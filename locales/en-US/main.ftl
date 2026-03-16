menu-file = File
menu-settings = Settings
menu-view = View
menu-info = Info
menu-about = About...
menu-save-params = Save as...
menu-load-params = Load...
menu-scale = UI Scale
menu-theme = Theme
menu-language = Language
lang-system = System
lang-english = English
lang-russian = Russian

view-left-panel = Show Table
view-right-panel = Show Parameters

theme-system = System
theme-light = Light
theme-dark = Dark
theme-egui = Egui
theme-breeze = Breeze
theme-solarized = Solarized

section-filter = Filter
section-result = Results
section-main-params = Main Parameters
section-recursive-params = Recursive Filter Parameters
section-design-params = Filter Design Parameters
section-operation-params = Filter Operation
section-plot = Plot

label-mode = Mode
label-filter-type = Filter Type
label-filter-category = Filter Category
label-task = Task
label-signal = Signal
label-table = Table
label-plot = Plot
label-legend = Legend
label-line-style = Style
label-line-spacing = Spacing
label-color = Color
label-plot-scale = Scale
label-line-width = Line Width

option-mode-design = Design
option-mode-operation = Operation
option-type-nonrecursive = Non-recursive
option-type-recursive = Recursive
option-category-lowpass = LPF
option-category-bandpass = BPF
option-category-bandstop = BSF
option-category-highpass = HPF
option-category-differentiating = DF

option-task-spectral = Spectral Analysis
option-task-diff = Differentiation
option-task-smooth = Smoothing

option-line-solid = Solid
option-line-dotted = Dotted
option-line-dashed = Dashed

plot-type-frequency = Magnitude Response
plot-type-phase = Phase Response
plot-type-impulse = Impulse Response
plot-type-step = Step Response
plot-type-operation = Output Signal

line-design-frequency = Magnitude Response
line-design-phase = Phase Response
line-design-impulse = Impulse Response
line-design-step = Step Response
line-operation-input = Input Signal
line-operation-output = Output Signal
line-operation-corrected = Corrected Signal
line-operation-reference = Reference Signal

table-type-frequency = Frequency Characteristics
table-type-impulse = Impulse Response
table-type-step = Step Response

button-save-csv = Save CSV
button-save-plot = Save Plot PNG
button-compute = Compute
button-reset-params = Reset Parameters
button-copy = Copy
button-cancel = Cancel
button-ok = OK

checkbox-auto-compute = Auto Compute

color-blue = Blue
color-orange = Orange
color-green = Green
color-red = Red
color-cyan = Cyan

plot-tooltip-zoom-out = Increase visible range.
plot-tooltip-fit = Auto-scale to current data.
plot-tooltip-zoom-in = Decrease visible range.

x-axis-frequency = Frequency ω, rad/s
x-axis-time = Time t, s
unit-seconds =  s
unit-rad-sec = rad/s
unit-px = px

msg-no-results = To display data, run computation first.
msg-about-authors = Authors:
msg-about-description = Filtr is an interactive DSP tool for filter design, signal processing, and response visualization.
msg-about-version = Version:
about-author-1 = > Alexander Begichev
about-author-2 = > Vladislav Zhizhin
about-author-3 = > Grigoriy Chikildin

msg-reset-confirm-title = Confirmation
msg-reset-confirm-body = Reset all parameters to default values?
msg-notice-title = Notice

msg-error-prefix = Error:
msg-compute-auto-disabled = { $error } Auto compute was disabled. Fix parameters and click "{ button-compute }".
msg-compute-failed-auto-disabled = Computation error: parameters are out of valid range. Auto compute was disabled.

err-dt-positive = Parameter Δt must be greater than 0.
err-dw-positive = Parameter Δω must be greater than 0.
err-nan-inf = Invalid parameter values (NaN/Inf).
err-v1 = Parameter V* must be greater than 1.
err-delta-range = Parameter δ must be in range (0, 1).
err-big-delta-range = Parameter Δ must be in range (0, 1).
err-design-freq-order = Design frequencies must satisfy: 0 ≤ ωs ≤ Ωφ.
err-kf-overflow = Δω is too small or Ωφ is too large: frequency index { $kf } exceeds { $max }.
err-nt-overflow = Transition process duration is too large: Nt={ $nt } exceeds { $max }.
err-ne-overflow = Signal duration is too large: Ne={ $ne } exceeds { $max }.
err-nx-lto-overflow = L/To are too large: Nx={ $nx } exceeds { $max }.
err-nx-tptf-overflow = Tp/Tf/To are too large: Nx={ $nx } exceeds { $max }.

table-col-frequency = Frequency
table-col-magnitude = Magnitude
table-col-phase = Phase
csv-header-frequency = N,Frequency,Magnitude,Phase

msg-params-saved = Parameters saved: { $path }
msg-params-loaded = Parameters loaded: { $path }
err-params-serialize = failed to serialize parameters: { $error }
err-params-decode = failed to decode RON: { $error }
err-params-write = failed to write file { $path }: { $error }
err-params-save = Error: failed to save parameters: { $error }
err-params-read = failed to read file { $path }: { $error }
err-params-invalid-ron = invalid RON in { $path }: { $error }
err-params-encoding = invalid file encoding: { $error }

msg-csv-saved = CSV saved: { $path }
msg-csv-saved-short = CSV saved.
msg-csv-cancelled = CSV saving canceled.
err-csv-write = Error: failed to write CSV { $path }: { $error }
err-csv-write-short = Error: failed to write CSV: { $error }

err-png-prepare = failed to prepare PNG: { $error }
err-plot-region = could not detect plot area. Open the plot on screen first.
msg-png-cancelled = PNG saving canceled.
err-png-save = failed to save PNG { $path }: { $error }
msg-plot-saved = Plot saved: { $path }
err-plot-region-short = Error: could not detect plot area.
msg-plot-saved-short = Plot saved.
err-png-save-short = Error: failed to save PNG: { $error }

filter-mode-tooltip = Application mode.
filter-type-tooltip = Filter structure: non-recursive or recursive.
filter-category-tooltip = Filter category: LPF, BPF, BSF, HPF, or differentiating.
dt-tooltip = Time sampling interval.

passband-ripple-tooltip = Magnitude-response ripple in passband.
stopband-ripple-tooltip = Magnitude-response ripple in stopband.
transition-band-tooltip = Parameter that defines transition-band width.
gain-tooltip = Passband gain coefficient.

cutoff-lower-tooltip = Lower cutoff frequency of filter passband.
cutoff-upper-tooltip = Upper cutoff frequency of filter passband.

task-tooltip = Select signal processing task.
main-interval-tooltip = Main interval duration.
impulse-len-tooltip = Impulse response length.
effective-len-tooltip = Effective duration of time response.
phase-delay-tooltip = Phase delay.
signal-tooltip = Select input signal.
noise-level-tooltip = Noise level.
noise-low-tooltip = Lower effective frequency of noise spectrum.
noise-high-tooltip = Upper effective frequency of noise spectrum.
noise-transition-tooltip = Transition-band width of noise spectrum.

recursive-level-tooltip = Level defining effective duration of time response.
recursive-duration-tooltip = Duration for determining time response.

dw-tooltip = Frequency sampling interval.
design-low-tooltip = Lower frequency bound for magnitude and phase response evaluation.
design-high-tooltip = Upper frequency bound for magnitude and phase response evaluation.
copy-tooltip = Copy value to clipboard.
