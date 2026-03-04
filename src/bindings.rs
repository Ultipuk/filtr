//! Mathematical core for filter synthesis/operation.
//!
//! This module is a behavior-preserving Rust port of the legacy Fortran code.
//! Key conventions:
//! - Arrays are allocated as length `CAP` and indexed as 1-based (`[1..]`).
//! - `iw` selects the program mode: `0` design, `1` operation.
//! - `il` selects filter implementation: `1` non-recursive, `2` recursive.
//! - `in_` selects filter class according to the original code mapping.
//!
//! The public entry point is [`norfil2`].

use crate::models::common::CommonParameters;
use crate::models::common_filter::CommonFilterParameters;
use crate::models::cutoff::CutoffParameters;
use crate::models::design::DesignParameters;
use crate::models::operation::OperationParameters;
use crate::models::recursive::RecursiveParameters;
use serde::{Deserialize, Serialize};

const PI: f64 = 3.14159265;
const CAP: usize = 5001; // 1-based indexing: valid data is [1..]

#[inline]
fn cabs(re: f64, im: f64) -> f64 {
    (re * re + im * im).sqrt()
}

/// Fortran N1ARND: linear congruential RNG used by the original code.
#[inline]
fn n1arnd(ix: i32) -> (i32, f64) {
    let mut iy = ix.wrapping_mul(65_539);
    if iy < 0 {
        iy = iy.wrapping_add(i32::MAX).wrapping_add(1);
    }
    let yfl = (iy as f64) * 0.4656613e-9;
    (iy, yfl)
}

/// Fortran N1YFBE: modified Bessel function approximation.
#[expect(clippy::indexing_slicing)]
fn n1yfbe(r: f64) -> f64 {
    let mut s = [0.0_f64; 25];
    s[0] = (r / 2.0).powi(2);

    let mut k = 1usize;
    while k < 25 {
        let kf = (k + 1) as f64;
        s[k] = s[k - 1] * (r / (2.0 * kf)).powi(2);
        if s[k] <= 1e-6 {
            break;
        }
        k += 1;
    }

    let mut f = 1.0;
    for term in s.iter().take(k + 1) {
        f += *term;
    }
    f
}

/// Fortran N1AFDS: generation of band-limited random signal.
#[expect(clippy::too_many_arguments)]
#[expect(clippy::indexing_slicing)]
fn n1afds(
    on: f64,
    ox: f64,
    db: f64,
    dt: f64,
    pz: f64,
    iv: &mut i32,
    nz: i32,
    l: &mut i32,
    w: &mut [f64],
    nv: &mut i32,
    z: &mut [f64],
) {
    if pz == 0.0 {
        for item in z.iter_mut().take(nz.max(0) as usize + 1).skip(1) {
            *item = 0.0;
        }
        return;
    }

    *l = (13.632812 / (dt * db) + 1.0) as i32;
    let m = *l + 1;

    let os = (on + 0.5 * db) * dt;
    let of_ = (ox - 0.5 * db) * dt;
    let wo = 0.318310 * (of_ - os);

    let fb = n1yfbe(6.0493);
    let d = 0.318310 / fb;

    for i in 1..=*l {
        let fi = 6.0493 * (1.0 - ((i as f64) / (*l as f64)).powi(2)).sqrt();
        let f = n1yfbe(fi);
        w[(m - i) as usize] =
            d * f * (((i as f64) * of_).sin() - ((i as f64) * os).sin()) / i as f64;
    }

    *nv = nz + 2 * *l + 1;

    let mut rng = *iv;
    for k in 1..=*nv {
        let (next, s) = n1arnd(rng);
        rng = next;
        z[k as usize] = 1.0 - 2.0 * s;
    }
    *iv = rng;

    let mut zm = 0.0;
    for k in 1..=nz {
        let j = k + m;
        let mut zj = wo * z[j as usize];

        for i in 1..=*l {
            let ir = m - i;
            zj += w[i as usize] * (z[(j + ir) as usize] + z[(j - ir) as usize]);
        }

        z[k as usize] = zj;
        let az = zj.abs();
        if zm <= az {
            zm = az;
        }
    }

    let pp = pz / zm;
    for k in 1..=nz {
        z[k as usize] *= pp;
    }
}

#[derive(Clone, Copy)]
struct NonRecursiveBandParams {
    transition_bandwidth: f64,
    lower_band_edge_dt: f64,
    upper_band_edge_dt: f64,
    center_weight: f64,
}

fn nonrecursive_band_params(
    filter_class: i32,
    v1: f64,
    ox: f64,
    on: f64,
    fk: f64,
    dt: f64,
) -> Option<NonRecursiveBandParams> {
    let nyquist_omega = PI / dt;
    let normalized_gain = fk / nyquist_omega;

    let params = match filter_class {
        1 => {
            let transition_bandwidth = (v1 - 1.0) * ox;
            NonRecursiveBandParams {
                transition_bandwidth,
                lower_band_edge_dt: 0.0,
                upper_band_edge_dt: (ox + 0.5 * transition_bandwidth) * dt,
                center_weight: normalized_gain * (ox + 0.5 * transition_bandwidth),
            }
        }
        2 => {
            let transition_bandwidth = (v1 - 1.0) * (ox - on);
            NonRecursiveBandParams {
                transition_bandwidth,
                lower_band_edge_dt: (on - 0.5 * transition_bandwidth) * dt,
                upper_band_edge_dt: (ox + 0.5 * transition_bandwidth) * dt,
                center_weight: normalized_gain * (ox - on + transition_bandwidth),
            }
        }
        3 => {
            let transition_bandwidth = (1.0 - 1.0 / v1) * (on - ox);
            NonRecursiveBandParams {
                transition_bandwidth,
                lower_band_edge_dt: (on - 0.5 * transition_bandwidth) * dt,
                upper_band_edge_dt: (ox + 0.5 * transition_bandwidth) * dt,
                center_weight: normalized_gain * (nyquist_omega + ox - on + transition_bandwidth),
            }
        }
        4 => {
            let transition_bandwidth = (1.0 - 1.0 / v1) * on;
            NonRecursiveBandParams {
                transition_bandwidth,
                lower_band_edge_dt: (on - 0.5 * transition_bandwidth) * dt,
                upper_band_edge_dt: 0.0,
                center_weight: normalized_gain * (nyquist_omega - on + 0.5 * transition_bandwidth),
            }
        }
        5 => {
            let transition_bandwidth = (v1 - 1.0) * ox;
            NonRecursiveBandParams {
                transition_bandwidth,
                lower_band_edge_dt: 0.0,
                upper_band_edge_dt: (ox + 0.5 * transition_bandwidth) * dt,
                center_weight: 0.0,
            }
        }
        _ => return None,
    };

    Some(params)
}

fn window_shape_constants(dl: f64) -> (f64, f64) {
    if dl < 0.01 {
        (4.41, 0.375)
    } else {
        (0.34, 1.61)
    }
}

#[expect(clippy::indexing_slicing)]
fn compute_relative_errors(
    reference_signal: &[f64],
    direct_output: &[f64],
    corrected_output: &[f64],
    ne: i32,
) -> (f64, f64) {
    let mut reference_energy = 0.0;
    let mut direct_error_energy = 0.0;
    let mut corrected_error_energy = 0.0;

    for k in 1..=ne {
        reference_energy += reference_signal[k as usize].powi(2);
        direct_error_energy += (reference_signal[k as usize] - direct_output[k as usize]).powi(2);
        corrected_error_energy +=
            (reference_signal[k as usize] - corrected_output[k as usize]).powi(2);
    }

    (
        (direct_error_energy / reference_energy).sqrt(),
        (corrected_error_energy / reference_energy).sqrt(),
    )
}

#[expect(clippy::indexing_slicing)]
fn compute_shifted_relative_errors(
    reference_signal: &[f64],
    raw_output: &[f64],
    corrected_output: &mut [f64],
    ne: i32,
    output_shift: i32,
    corrected_shift: i32,
) -> (f64, f64) {
    let mut reference_energy = 0.0;
    let mut raw_error_energy = 0.0;
    let mut corrected_error_energy = 0.0;

    for k in 1..=ne {
        corrected_output[k as usize] = raw_output[(k + output_shift + corrected_shift) as usize];
        raw_error_energy +=
            (reference_signal[k as usize] - raw_output[(k + output_shift) as usize]).powi(2);
        corrected_error_energy +=
            (reference_signal[k as usize] - corrected_output[k as usize]).powi(2);
        reference_energy += reference_signal[k as usize].powi(2);
    }

    (
        (raw_error_energy / reference_energy).sqrt(),
        (corrected_error_energy / reference_energy).sqrt(),
    )
}

#[expect(clippy::indexing_slicing)]
fn mirror_nonrecursive_kernel(filter_class: i32, l: i32, l1: i32, w: &mut [f64]) {
    for k in 1..=l {
        w[(l1 + k) as usize] = w[(l1 - k) as usize];
        if filter_class == 5 {
            w[(l1 + k) as usize] = -w[(l1 - k) as usize];
        }
    }
}

#[expect(clippy::indexing_slicing)]
fn fill_operation_signals(
    signal_mode: i32,
    signal_variant: i32,
    dt: f64,
    nf: i32,
    ne: i32,
    nx: i32,
    px: f64,
    o1: f64,
    o2: f64,
    db: f64,
    w: &mut [f64],
    x: &mut [f64],
    v: &mut [f64],
) {
    if signal_mode == 1 {
        let omega2 = 4.0 * dt;
        let omega3 = 7.0 * dt;

        for k in 1..=ne {
            let sample_index = nf + k;
            let sample_time = (sample_index as f64) * dt;
            v[k as usize] = match signal_variant {
                1 => sample_time.sin(),
                2 => sample_time.sin() + (sample_index as f64 * omega2).sin(),
                3 => sample_time.sin() + (sample_index as f64 * omega3).sin(),
                4 => (sample_index as f64 * omega2).sin(),
                5 => (sample_index as f64 * omega2).sin() + (sample_index as f64 * omega3).sin(),
                6 => (sample_index as f64 * omega3).sin(),
                _ => 0.0,
            };
        }

        for k in 1..=nx {
            let k0 = (k - 1) as f64;
            x[k as usize] = (k0 * dt).sin() + (k0 * omega2).sin() + (k0 * omega3).sin();
        }
        return;
    }

    let mut random_seed = 21733;
    let mut random_kernel_len = 0;
    let mut random_buf_size = 0;
    n1afds(
        o1,
        o2,
        db,
        dt,
        px,
        &mut random_seed,
        nx,
        &mut random_kernel_len,
        w,
        &mut random_buf_size,
        x,
    );

    for k in 1..=ne {
        v[k as usize] = (((nf + k) as f64) * dt).sin();
    }

    if signal_mode == 3 {
        for k in 1..=nx {
            x[k as usize] += 1.0 - (((k - 1) as f64) * dt).cos();
        }
    } else {
        for k in 1..=nx {
            x[k as usize] += (((k - 1) as f64) * dt).sin();
        }
    }
}

fn setup_design_mode_ranges(
    il: i32,
    os: f64,
    of_: f64,
    df: f64,
    th: f64,
    dt: f64,
    nt: &mut i32,
) -> (i32, i32) {
    let ks = (os / df + 1.1) as i32;
    let kf = (of_ / df + 1.1) as i32;
    if il != 1 {
        *nt = (th / dt + 1.1) as i32;
    }
    (ks, kf)
}

fn setup_operation_mode_lengths(
    il: i32,
    dt: f64,
    to: f64,
    l: i32,
    tp: f64,
    tf: f64,
    ne: &mut i32,
    nf: &mut i32,
    nx: &mut i32,
    nt: &mut i32,
) -> i32 {
    *ne = (to / dt + 1.1) as i32;
    *nt = 0;

    if il != 2 {
        *nf = 2 * l;
        *nx = *nf + l + *ne + 1;
        return 0;
    }

    *nf = (tp / dt + 1.1) as i32;
    let lf = (tf / dt + 0.1) as i32;
    *nx = *nf + lf.abs() + *ne + 1;
    lf
}

#[expect(clippy::too_many_arguments)]
#[expect(clippy::indexing_slicing)]
fn norfil(
    dl: f64,
    v1: f64,
    ox: f64,
    on: f64,
    fk: f64,
    dt: f64,
    in_: i32,
    iw: i32,
    df: f64,
    ks: i32,
    kf: i32,
    nx: i32,
    x: &mut [f64],
    ne: i32,
    v: &mut [f64],
    r: &mut f64,
    l: &mut i32,
    l1: &mut i32,
    w: &mut [f64],
    a: &mut [f64],
    f: &mut [f64],
    y: &mut [f64],
    z: &mut [f64],
    ey: &mut f64,
    ez: &mut f64,
) {
    // Non-recursive filter branch:
    // 1) Build impulse response coefficients `w`
    // 2) Optionally compute frequency response (design mode)
    // 3) Filter input signal and evaluate operation errors
    let Some(band) = nonrecursive_band_params(in_, v1, ox, on, fk, dt) else {
        return;
    };
    let (window_c1, window_c2) = window_shape_constants(dl);

    *r = ((window_c1 * window_c1 - 4.0 * window_c2 * (19.6 + 20.0 * dl.log10())).sqrt()
        - window_c1)
        / (2.0 * window_c2);
    *l = (1.0 + 2.0 * ((*r * *r + PI * PI).sqrt()) / (band.transition_bandwidth * dt)) as i32;

    let fb = n1yfbe(*r);
    let mut da = fk / (PI * fb);
    if in_ == 5 {
        da = 1.0 / (PI * dt * fb);
    }

    *l1 = *l + 1;

    for i in 1..=*l {
        let fr = *r * (1.0 - ((i as f64) / (*l as f64)).powi(2)).sqrt();
        let fi = n1yfbe(fr);

        let d = if in_ == 5 {
            (da * fi / (i as f64))
                * ((((i as f64) * band.upper_band_edge_dt).sin())
                    - (i as f64)
                        * band.upper_band_edge_dt
                        * (((i as f64) * band.upper_band_edge_dt).cos()))
                / (i as f64)
        } else {
            da * fi
                * ((((i as f64) * band.upper_band_edge_dt).sin())
                    - (((i as f64) * band.lower_band_edge_dt).sin()))
                / (i as f64)
        };

        w[(*l1 - i) as usize] = d;
    }

    w[*l1 as usize] = band.center_weight;

    if iw != 1 {
        let pi2 = PI / 2.0;
        let dd = df * dt;

        for k in ks..=kf {
            let om = ((k - 1) as f64) * dd;
            let mut af = band.center_weight;
            let ff;

            if in_ == 5 {
                ff = pi2 - (*l as f64) * om;
                for i in 1..=*l {
                    af += 2.0 * w[(*l1 - i) as usize] * (((i as f64) * om).sin());
                }
            } else {
                ff = -(*l as f64) * om;
                for i in 1..=*l {
                    af += 2.0 * w[(*l1 - i) as usize] * (((i as f64) * om).cos());
                }
            }

            a[k as usize] = af.abs();
            f[k as usize] = ff;
        }

        if iw == 0 {
            return;
        }
    }

    let filter_delay = 2 * *l + 1;
    let output_len = ne + *l;
    let signal_parity = if in_ == 5 { -1.0 } else { 1.0 };

    for k in 1..=output_len {
        let shifted_index = filter_delay + k;
        let mut yo = w[*l1 as usize] * x[(shifted_index - *l) as usize];
        for i in 1..=*l {
            yo += w[i as usize]
                * (x[(shifted_index + 1 - i) as usize] + signal_parity * x[(k + i) as usize]);
        }
        y[k as usize] = yo;
        if k > *l {
            z[(k - *l) as usize] = yo;
        }
    }

    (*ey, *ez) = compute_relative_errors(v, y, z, ne);
    let _ = nx;
}

#[expect(clippy::too_many_arguments)]
#[expect(clippy::indexing_slicing)]
fn recfil(
    dm: f64,
    dl: f64,
    v1: f64,
    ox: f64,
    on: f64,
    fk: f64,
    dt: f64,
    in_: i32,
    iw: i32,
    df: f64,
    ks: i32,
    kf: i32,
    dh: f64,
    nt: i32,
    nx: &mut i32,
    ne: i32,
    lf: &mut i32,
    nf: &mut i32,
    l: &mut i32,
    lo: &mut i32,
    n: &mut i32,
    ey: &mut f64,
    ez: &mut f64,
    x: &mut [f64],
    v: &mut [f64],
    a1: &mut [f64],
    a2: &mut [f64],
    ak: &mut [f64],
    c1: &mut [f64],
    c2: &mut [f64],
    c3: &mut [f64],
    u: &mut [f64],
    a: &mut [f64],
    f: &mut [f64],
    z: &mut [f64],
    y: &mut [f64],
) {
    // Recursive filter branch:
    // 1) Analog prototype synthesis
    // 2) Frequency-response evaluation (design mode)
    // 3) Bilinear transform to digital coefficients
    // 4) Time-domain simulation and error metrics
    let passband_ripple_ratio = dm * (2.0 - dm) / (1.0 - dm).powi(2);
    let prototype_quality = (passband_ripple_ratio * dl * dl) / (1.0 - dl * dl);
    *n = (1.0
        + (((1.0 + (1.0 - prototype_quality).sqrt()) / prototype_quality.sqrt()).ln())
            / (v1 + (v1 * v1 - 1.0).sqrt()).ln()) as i32;

    let mut id;
    let mut a10;
    let mut a20 = 0.0;
    let mut ok = 0.0;
    let mut oo = 0.0;
    let mut oa = 0.0;
    let mut ob = 0.0;

    loop {
        *l = *n / 2;
        id = if 2 * *l + 1 == *n { 1 } else { 0 };

        let d = ((2.0 - dm) / dm).powf(1.0 / (2.0 * (*n as f64)));
        let ao = (d - 1.0 / d) / 2.0;

        for i in 1..=*l {
            let aa = 0.5 * (((2 * i - 1) as f64) * PI / (*n as f64));
            a1[i as usize] = 0.5 * (d - 1.0 / d) * aa.sin();
            a2[i as usize] = 0.5 * (d + 1.0 / d) * aa.cos();
            ak[i as usize] = a1[i as usize].powi(2) + a2[i as usize].powi(2);
        }

        let mut of_ = 0.0;
        let mut os = 0.0;

        if in_ != 4 {
            of_ = 2.0 * (ox * dt / 2.0).sin() / (ox * dt / 2.0).cos() / dt;
        }
        if in_ != 1 {
            os = 2.0 * (on * dt / 2.0).sin() / (on * dt / 2.0).cos() / dt;
        }

        if in_ == 2 || in_ == 3 {
            let m = *l;
            *l = 2 * m;

            oo = (os * of_).sqrt();
            let od = (of_ - os).abs();
            ok = os * of_;
            let oc = (2.0 * oo / od).powi(2);

            if in_ == 2 {
                oa = od;
                ob = oc;
            }

            for i in 1..=m {
                let i2 = 2 * i;
                let i1 = i2 - 1;

                if in_ == 3 {
                    oa = od / ak[i as usize];
                    ob = oc * ak[i as usize].powi(2);
                }

                let sk = ob + a2[i as usize].powi(2) - a1[i as usize].powi(2);
                let so = (sk.powi(2) + (2.0 * a1[i as usize] * a2[i as usize]).powi(2)).sqrt();
                let da = (0.5 * (so - sk).abs()).sqrt();
                let db = (0.5 * (so + sk)).sqrt();

                c1[i1 as usize] = 0.5 * (a1[i as usize] - da) * oa;
                c1[i2 as usize] = 0.5 * (a1[i as usize] + da) * oa;
                c2[i1 as usize] = 0.5 * (db - a2[i as usize]) * oa;
                c2[i2 as usize] = 0.5 * (db + a2[i as usize]) * oa;

                if in_ != 3 {
                    c3[i1 as usize] = ak[i as usize].sqrt() * od;
                    c3[i2 as usize] = c3[i1 as usize];
                }
            }

            for i in 1..=*l {
                if in_ != 3 {
                    ak[i as usize] = c3[i as usize];
                }
                a1[i as usize] = c1[i as usize];
                a2[i as usize] = c2[i as usize];
            }

            if id == 1 {
                let mut a10_local = 0.0;
                if in_ == 2 {
                    a10_local = 0.5 * od * ao;
                }
                if in_ == 3 {
                    a10_local = 0.5 * od / ao;
                }
                let ap = ok - a10_local * a10_local;
                if ap <= 0.0 {
                    *n += 1;
                    continue;
                }
                a10 = a10_local;
                a20 = ap.sqrt();
            } else {
                a10 = 0.0;
            }
        } else {
            for i in 1..=*l {
                if in_ == 4 {
                    a1[i as usize] = a1[i as usize] * os / ak[i as usize];
                    a2[i as usize] = a2[i as usize] * os / ak[i as usize];
                } else {
                    ak[i as usize] *= of_ * of_;
                    a1[i as usize] *= of_;
                    a2[i as usize] *= of_;
                }
            }

            if id == 1 {
                if in_ == 1 {
                    a10 = ao * of_;
                } else if in_ == 4 {
                    a10 = os / ao;
                } else {
                    a10 = 0.0;
                }
            } else {
                a10 = 0.0;
            }
        }

        // 3. Frequency response
        if iw != 1 {
            for k in ks..=kf {
                a[k as usize] = fk;
                if id == 0 {
                    a[k as usize] = (1.0 - dm) * fk;
                }
                f[k as usize] = 0.0;

                let om = (2.0 / dt) * (((k - 1) as f64) * df * dt / 2.0).tan();

                if id == 1 {
                    let (ac, fc) = if in_ == 2 || in_ == 3 {
                        let ac = cabs(a10 * a10 + a20 * a20 - om * om, 2.0 * a10 * om);
                        let mut fc = -((om + a20) / a10).atan() - ((om - a20) / a10).atan();

                        if in_ == 3 {
                            let amp = if (om - oo).abs() < f64::EPSILON {
                                0.0
                            } else {
                                (oo * oo - om * om) / ac
                            };

                            if (om - oo).abs() < f64::EPSILON {
                                fc += 0.5 * PI;
                            }
                            if om > oo {
                                fc += PI;
                            }
                            (amp.abs(), fc)
                        } else {
                            fc += 0.5 * PI;
                            (2.0 * a10 * om / ac, fc)
                        }
                    } else {
                        let ac = cabs(a10, om);
                        let mut fc = -((om / a10).atan());
                        let amp = if in_ == 1 { a10 / ac } else { om / ac };
                        if in_ == 4 {
                            fc += 0.5 * PI;
                        }
                        (amp, fc)
                    };

                    a[k as usize] *= ac;
                    f[k as usize] += fc;
                }

                let mut fc0 = 0.0;
                if in_ == 3 {
                    if om < oo {
                        fc0 = 0.0;
                    }
                    if (om - oo).abs() < f64::EPSILON {
                        fc0 = 0.5 * PI * (*l as f64);
                    }
                    if om > oo {
                        fc0 = PI * (*l as f64);
                    }
                } else {
                    if in_ == 1 {
                        fc0 = 0.0;
                    }
                    if in_ == 2 {
                        fc0 = 0.5 * PI * (*l as f64);
                    }
                    if in_ == 4 {
                        fc0 = PI * (*l as f64);
                    }
                }
                f[k as usize] += fc0;

                let mut ao_acc = 1.0;
                let mut fo_acc = 0.0;

                for i in 1..=*l {
                    let mut ac = cabs(
                        a1[i as usize] * a1[i as usize] + a2[i as usize] * a2[i as usize] - om * om,
                        2.0 * a1[i as usize] * om,
                    );

                    let fc = ((om + a2[i as usize]) / a1[i as usize]).atan()
                        + ((om - a2[i as usize]) / a1[i as usize]).atan();

                    if in_ == 1 {
                        ac = ak[i as usize] / ac;
                    }
                    if in_ == 2 {
                        ac = ak[i as usize] * om / ac;
                    }
                    if in_ == 3 {
                        ac = (oo * oo - om * om).abs() / ac;
                    }
                    if in_ == 4 {
                        ac = om * om / ac;
                    }

                    ao_acc *= ac;
                    fo_acc += fc;
                }

                a[k as usize] *= ao_acc;
                f[k as usize] -= fo_acc;
            }

            if in_ == 1 {
                os = 0.0;
            }
            if in_ == 4 {
                of_ = ((kf - 1) as f64) * df;
            }

            let mut k = 1;
            if in_ != 3 {
                if in_ == 2 || in_ == 4 {
                    k = (os / df + 1.05) as i32;
                }
                let kc = ((of_ + os) / (2.0 * df)) as i32;
                let mut ii = -1.0;
                if in_ == 4 {
                    ii = 0.0;
                }
                *lf =
                    ((f[kc as usize] - f[k as usize]) / (((kc - k) as f64) * df * dt) + ii) as i32;
                if in_ == 4 {
                    *lf = -*lf;
                }
            } else {
                let kc = (1.0 / df) as i32;
                let lf1 = (f[kc as usize] - f[k as usize]) / (((kc - k) as f64) * df * dt) - 1.05;
                k = (os / df + 2.05) as i32;
                let kc2 = (7.0 / df) as i32;
                let lf2 = (f[kc2 as usize] - f[k as usize]) / (((kc2 - k) as f64) * df * dt);
                *lf = f64::midpoint(lf1, lf2) as i32;
            }
        }

        // 4. Digital coefficients
        let mut af = fk;
        if id == 0 {
            af *= 1.0 - dm;
        }

        let dd = dt * dt;
        let (b1, b2);
        let mut b10 = 0.0;
        let mut b20 = 0.0;
        let mut ako = 0.0;

        if in_ == 2 || in_ == 3 {
            if in_ == 3 {
                oa = 4.0 + ok * dd;
                ob = 4.0 - ok * dd;
                b1 = -2.0 * ob / oa;
                b2 = 1.0;
            } else {
                b1 = 0.0;
                b2 = -1.0;
            }

            if id == 1 {
                let g = (a10 * a10 + a20 * a20) * dd;
                let da = 4.0 + 4.0 * a10 * dt + g;

                if in_ == 3 {
                    b10 = b1;
                    b20 = 1.0;
                    ako = oa / da;
                } else {
                    b10 = 0.0;
                    b20 = -1.0;
                    ako = 4.0 * a10 * dt / da;
                }

                a20 = (4.0 - 4.0 * a10 * dt + g) / da;
                a10 = -2.0 * (4.0 - g) / da;
            }
        } else {
            b1 = if in_ == 4 { -2.0 } else { 2.0 };
            b2 = 1.0;

            if id == 1 {
                let da = 2.0 + a10 * dt;
                b10 = if in_ == 4 { -1.0 } else { 1.0 };
                ako = if in_ == 4 { 2.0 / da } else { a10 * dt / da };
                a10 = -(2.0 - a10 * dt) / da;
            }
        }

        if in_ != 2 {
            let _d1 = -ob / oa;
            let _d2 = 4.0 * oo * dt / oa;
        }

        for i in 1..=*l {
            let g1 = 4.0 * a1[i as usize] * dt;
            let g2 = (a1[i as usize] * a1[i as usize] + a2[i as usize] * a2[i as usize]) * dd;
            let da = 4.0 + g1 + g2;

            a1[i as usize] = -2.0 * (4.0 - g2) / da;
            a2[i as usize] = (4.0 - g1 + g2) / da;

            if in_ == 1 {
                ak[i as usize] = ak[i as usize] * dd / da;
            }
            if in_ == 2 {
                ak[i as usize] = ak[i as usize] * dt * 2.0 / da;
            }
            if in_ == 3 {
                ak[i as usize] = oa / da;
            }
            if in_ == 4 {
                ak[i as usize] = 4.0 / da;
            }

            if iw != 1 {
                c1[i as usize] = a1[i as usize] / 2.0;
                c2[i as usize] = (a2[i as usize] - c1[i as usize] * c1[i as usize])
                    .abs()
                    .sqrt();
            }
        }

        // 5. Transient / impulse response in design mode
        if iw != 1 {
            for k in 1..=nt {
                x[k as usize] = 1.0;
                if in_ == 2 {
                    x[k as usize] = (k - 1) as f64;
                }
                if in_ == 4 {
                    x[k as usize] = 0.5 * ((k - 1) as f64).powi(2);
                }
            }
            *nx = nt;
        }

        *lo = 3 * *l + 3;
        *n = 2 * *l;
        if id == 1 {
            if in_ == 1 || in_ == 4 {
                *n += 1;
            }
            if in_ == 2 || in_ == 3 {
                *n += 2;
            }
        }

        let n1 = *n + 1;
        let mut k = 0;
        loop {
            k += 1;
            if k > *nx {
                break;
            }

            if k <= *n {
                y[k as usize] = 0.0;
                continue;
            }

            if k <= n1 {
                if id == 1 {
                    u[1] = 0.0;
                    u[2] = 0.0;
                } else {
                    u[1] = x[(k - 2) as usize];
                    u[2] = x[(k - 1) as usize];
                }

                for i in 1..=*l {
                    let j = 3 * i + 3;
                    u[(j - 1) as usize] = 0.0;
                    u[(j - 2) as usize] = 0.0;
                }
            }

            if id == 1 {
                if in_ == 2 || in_ == 3 {
                    u[3] = ako
                        * (x[k as usize] + b10 * x[(k - 1) as usize] + b20 * x[(k - 2) as usize])
                        - (a10 * u[2] + a20 * u[1]);
                } else {
                    u[3] = ako * (x[k as usize] + b10 * x[(k - 1) as usize]) - a10 * u[2];
                }
            } else {
                u[3] = x[k as usize];
            }

            for i in 1..=*l {
                let j = 3 * i + 3;
                u[j as usize] = ak[i as usize]
                    * (u[(j - 3) as usize] + b1 * u[(j - 4) as usize] + b2 * u[(j - 5) as usize])
                    - (a1[i as usize] * u[(j - 1) as usize] + a2[i as usize] * u[(j - 2) as usize]);
            }

            u[1] = u[2];
            u[2] = u[3];
            for i in 1..=*l {
                let j = 3 * i + 3;
                u[(j - 2) as usize] = u[(j - 1) as usize];
                u[(j - 1) as usize] = u[j as usize];
            }

            y[k as usize] = af * u[*lo as usize];
        }

        if iw != 1 {
            let nh = nt / 3;
            let nh1 = 2 * nh;
            let mut ym = y[1].abs();
            let mut sh = 0.0;

            for k in 1..=nt {
                let ya = y[k as usize].abs();
                if ym < ya {
                    ym = ya;
                }
                if k >= nh1 {
                    sh += y[k as usize];
                }
            }

            let yh = dh * ym;
            sh /= nh as f64;
            let yh1 = sh + yh;
            let yh2 = sh - yh;

            *nf = nt;
            let mut k = nt;
            loop {
                if y[k as usize] < yh1 && y[k as usize] > yh2 {
                    k -= 1;
                    if k <= 0 {
                        break;
                    }
                } else {
                    *nf = k;
                    break;
                }
            }
        }

        if iw != 0 {
            (*ey, *ez) = compute_shifted_relative_errors(v, y, z, ne, *nf, *lf);
        }

        break;
    }
}

#[expect(clippy::too_many_arguments)]
fn norecfil(
    iw: &mut i32,
    il: &mut i32,
    in_: &mut i32,
    dt: &mut f64,
    df: &mut f64,
    os: &mut f64,
    of_: &mut f64,
    dh: &mut f64,
    th: &mut f64,
    im: &mut i32,
    to: &mut f64,
    l: &mut i32,
    tp: &mut f64,
    tf: &mut f64,
    px: &mut f64,
    o1: &mut f64,
    o2: &mut f64,
    db: &mut f64,
    dm: &mut f64,
    dl: &mut f64,
    v1: &mut f64,
    ox: &mut f64,
    on: &mut f64,
    fk: &mut f64,
    iv: &mut i32,
    x: &mut [f64],
    w: &mut [f64],
    v: &mut [f64],
    a1: &mut [f64],
    a2: &mut [f64],
    ak: &mut [f64],
    c1: &mut [f64],
    c2: &mut [f64],
    c3: &mut [f64],
    u: &mut [f64],
    a: &mut [f64],
    f: &mut [f64],
    z: &mut [f64],
    y: &mut [f64],
    ey: &mut f64,
    ez: &mut f64,
    r: &mut f64,
    l1: &mut i32,
    nx: &mut i32,
    ne: &mut i32,
    kn: &mut i32,
    n: &mut i32,
    lo: &mut i32,
    nf: &mut i32,
    nt: &mut i32,
) {
    // Top-level dispatcher matching the original `NORECFIL` flow.
    // Chooses design/operation preparation, then routes to
    // non-recursive (`norfil`) or recursive (`recfil`) implementation.
    let (ks, kf);
    let mut lf = 0;

    if *iw != 1 {
        (ks, kf) = setup_design_mode_ranges(*il, *os, *of_, *df, *th, *dt, nt);
    } else {
        ks = 0;
        kf = 0;
        lf = setup_operation_mode_lengths(*il, *dt, *to, *l, *tp, *tf, ne, nf, nx, nt);
        fill_operation_signals(*im, *iv, *dt, *nf, *ne, *nx, *px, *o1, *o2, *db, w, x, v);
    }

    if *il != 2 {
        norfil(
            *dl, *v1, *ox, *on, *fk, *dt, *in_, *iw, *df, ks, kf, *nx, x, *ne, v, r, l, l1, w, a,
            f, y, z, ey, ez,
        );

        if *iw == 1 {
            *kn = 1;
        } else {
            *l1 = *l + 1;
            mirror_nonrecursive_kernel(*in_, *l, *l1, w);
        }
    } else {
        recfil(
            *dm, *dl, *v1, *ox, *on, *fk, *dt, *in_, *iw, *df, ks, kf, *dh, *nt, nx, *ne, &mut lf,
            nf, l, lo, n, ey, ez, x, v, a1, a2, ak, c1, c2, c3, u, a, f, z, y,
        );

        if *iw == 1 {
            *kn = *nf;
        } else {
            *tp = ((*nf - 1) as f64) * *dt;
            *tf = -((lf as f64) * *dt);
        }
    }
}

/// Computed arrays and scalar metrics produced by the mathematical core.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Outputs {
    pub a: Vec<f64>,
    pub x: Vec<f64>,
    pub w: Vec<f64>,
    pub v: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub f: Vec<f64>,
    pub r: f64,
    pub l: i32,
    pub l1: i32,
    pub nx: i32,
    pub ne: i32,
    pub nf: i32,
    pub nt: i32,
    pub ey: f64,
    pub ez: f64,
    pub tp: f64,
    pub tf: f64,
    pub n: i32,
}

/// Snapshot of user-selected input parameters used for a run.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Inputs {
    pub design_parameters: DesignParameters,
    pub common_parameters: CommonParameters,
    pub common_filter_parameters: CommonFilterParameters,
    pub cutoff_parameters: CutoffParameters,
    pub operation_parameters: OperationParameters,
    pub recursive_parameters: RecursiveParameters,
}

/// Full result object returned by [`norfil2`].
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Results {
    pub inputs: Inputs,
    pub outputs: Outputs,
}

/// Mutable scalar state consumed by the mathematical core (`norecfil`).
///
/// Execution model:
/// 1. Build from UI/model parameters.
/// 2. Run the core routine once with scratch buffers.
/// 3. Convert finalized state to `Outputs`.
#[derive(Clone, Debug)]
struct CoreState {
    iw: i32,
    il: i32,
    in_: i32,
    dt: f64,
    df: f64,
    os: f64,
    of_: f64,
    dh: f64,
    th: f64,
    im: i32,
    to: f64,
    l: i32,
    tp: f64,
    tf: f64,
    px: f64,
    o1: f64,
    o2: f64,
    db: f64,
    dm: f64,
    dl: f64,
    v1: f64,
    ox: f64,
    on: f64,
    fk: f64,
    iv: i32,
    ey: f64,
    ez: f64,
    r: f64,
    l1: i32,
    nx: i32,
    ne: i32,
    kn: i32,
    n: i32,
    lo: i32,
    nf: i32,
    nt: i32,
}

impl CoreState {
    fn from_model(
        common_parameters: &CommonParameters,
        design_parameters: &DesignParameters,
        common_filter_parameters: &CommonFilterParameters,
        cutoff_parameters: &CutoffParameters,
        recursive_parameters: &RecursiveParameters,
        operation_parameters: &OperationParameters,
    ) -> Self {
        Self {
            iw: common_parameters.get_iw(),
            il: common_parameters.get_il(),
            in_: common_parameters.get_in(),
            dt: common_parameters.get_dt(),
            df: design_parameters.get_df(),
            os: design_parameters.get_os(),
            of_: design_parameters.get_of(),
            dh: recursive_parameters.get_dh(),
            th: recursive_parameters.get_th(),
            im: operation_parameters.get_im() as i32,
            to: operation_parameters.get_to(),
            l: operation_parameters.get_l(),
            tp: operation_parameters.get_tp(),
            tf: operation_parameters.get_tf(),
            iv: operation_parameters.get_iv(),
            px: operation_parameters.conv_diff_parameters.get_px(),
            o1: operation_parameters.conv_diff_parameters.get_o1(),
            o2: operation_parameters.conv_diff_parameters.get_o2(),
            db: operation_parameters.conv_diff_parameters.get_db(),
            dm: common_filter_parameters.get_dm(),
            dl: common_filter_parameters.get_dl(),
            v1: common_filter_parameters.get_v1(),
            ox: cutoff_parameters.get_ox(),
            on: cutoff_parameters.get_on(),
            fk: common_filter_parameters.get_fk(),
            ey: 0.0,
            ez: 0.0,
            r: 0.0,
            l1: 0,
            nx: 0,
            ne: 0,
            kn: 0,
            n: 0,
            lo: 0,
            nf: 0,
            nt: 0,
        }
    }

    fn to_outputs(&self, buffers: CoreBuffers) -> Outputs {
        Outputs {
            a: buffers.a,
            x: buffers.x,
            w: buffers.w,
            v: buffers.v,
            y: buffers.y,
            z: buffers.z,
            f: buffers.f,
            r: self.r,
            l: self.l,
            l1: self.l1,
            nx: self.nx,
            ne: self.ne,
            nf: self.nf,
            nt: self.nt,
            ey: self.ey,
            ez: self.ez,
            tp: self.tp,
            tf: self.tf,
            n: self.n,
        }
    }
}

/// Scratch arrays for the Fortran-compatible algorithms.
///
/// Important: arrays are intentionally treated as 1-based (`[1..]`) for parity.
#[derive(Clone, Debug)]
struct CoreBuffers {
    x: Vec<f64>,
    w: Vec<f64>,
    v: Vec<f64>,
    a1: Vec<f64>,
    a2: Vec<f64>,
    ak: Vec<f64>,
    c1: Vec<f64>,
    c2: Vec<f64>,
    c3: Vec<f64>,
    u: Vec<f64>,
    a: Vec<f64>,
    f: Vec<f64>,
    z: Vec<f64>,
    y: Vec<f64>,
}

impl CoreBuffers {
    fn new() -> Self {
        Self {
            x: vec![0.0; CAP],
            w: vec![0.0; CAP],
            v: vec![0.0; CAP],
            a1: vec![0.0; CAP],
            a2: vec![0.0; CAP],
            ak: vec![0.0; CAP],
            c1: vec![0.0; CAP],
            c2: vec![0.0; CAP],
            c3: vec![0.0; CAP],
            u: vec![0.0; CAP],
            a: vec![0.0; CAP],
            f: vec![0.0; CAP],
            z: vec![0.0; CAP],
            y: vec![0.0; CAP],
        }
    }
}

fn run_core(state: &mut CoreState, buffers: &mut CoreBuffers) {
    norecfil(
        &mut state.iw,
        &mut state.il,
        &mut state.in_,
        &mut state.dt,
        &mut state.df,
        &mut state.os,
        &mut state.of_,
        &mut state.dh,
        &mut state.th,
        &mut state.im,
        &mut state.to,
        &mut state.l,
        &mut state.tp,
        &mut state.tf,
        &mut state.px,
        &mut state.o1,
        &mut state.o2,
        &mut state.db,
        &mut state.dm,
        &mut state.dl,
        &mut state.v1,
        &mut state.ox,
        &mut state.on,
        &mut state.fk,
        &mut state.iv,
        &mut buffers.x,
        &mut buffers.w,
        &mut buffers.v,
        &mut buffers.a1,
        &mut buffers.a2,
        &mut buffers.ak,
        &mut buffers.c1,
        &mut buffers.c2,
        &mut buffers.c3,
        &mut buffers.u,
        &mut buffers.a,
        &mut buffers.f,
        &mut buffers.z,
        &mut buffers.y,
        &mut state.ey,
        &mut state.ez,
        &mut state.r,
        &mut state.l1,
        &mut state.nx,
        &mut state.ne,
        &mut state.kn,
        &mut state.n,
        &mut state.lo,
        &mut state.nf,
        &mut state.nt,
    );
}

/// Computes filter characteristics/signals for the current parameter set.
///
/// Inputs are read from model structs, executed through the Fortran-compatible
/// core, then returned as `Results { inputs, outputs }`.
pub fn norfil2(
    common_parameters: &mut CommonParameters,
    design_parameters: &mut DesignParameters,
    common_filter_parameters: &mut CommonFilterParameters,
    cutoff_parameters: &mut CutoffParameters,
    recursive_parameters: &mut RecursiveParameters,
    operation_parameters: &mut OperationParameters,
) -> Results {
    let mut state = CoreState::from_model(
        common_parameters,
        design_parameters,
        common_filter_parameters,
        cutoff_parameters,
        recursive_parameters,
        operation_parameters,
    );
    let mut buffers = CoreBuffers::new();
    run_core(&mut state, &mut buffers);

    let outputs = state.to_outputs(buffers);

    let inputs = Inputs {
        design_parameters: design_parameters.clone(),
        common_parameters: common_parameters.clone(),
        common_filter_parameters: common_filter_parameters.clone(),
        cutoff_parameters: cutoff_parameters.clone(),
        operation_parameters: operation_parameters.clone(),
        recursive_parameters: recursive_parameters.clone(),
    };

    Results { inputs, outputs }
}
