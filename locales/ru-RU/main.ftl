menu-file = Файл
menu-settings = Настройки
menu-view = Вид
menu-info = Информация
menu-about = О программе...
menu-save-params = Сохранить как...
menu-load-params = Загрузить...
menu-scale = Масштаб
menu-theme = Тема
menu-language = Язык
lang-system = Системный
lang-english = English
lang-russian = Русский

view-left-panel = Показать таблицу
view-right-panel = Показать параметры

theme-system = Системная
theme-light = Светлая
theme-dark = Тёмная
theme-egui = Egui
theme-breeze = Breeze
theme-solarized = Solarized

section-filter = Фильтр
section-result = Результат
section-main-params = Основные параметры
section-recursive-params = Параметры рекурсивного фильтра
section-design-params = Параметры проектирования фильтра
section-operation-params = Функционирование фильтра
section-plot = График

label-mode = Режим
label-filter-type = Вид фильтра
label-filter-category = Тип фильтра
label-task = Задача
label-signal = Сигнал
label-table = Таблица
label-plot = График
label-legend = Легенда
label-line-style = Стиль
label-line-spacing = Интервал
label-color = Цвет
label-plot-scale = Масштаб
label-line-width = Толщина линий

option-mode-design = Проектирование
option-mode-operation = Функционирование
option-type-nonrecursive = Нерекурсивный
option-type-recursive = Рекурсивный
option-category-lowpass = НЧФ
option-category-bandpass = ПФ
option-category-bandstop = РФ
option-category-highpass = ВЧФ
option-category-differentiating = ДФ

option-task-spectral = Спектральный анализ
option-task-diff = Дифференцирование
option-task-smooth = Сглаживание

option-line-solid = Сплошная
option-line-dotted = Пунктир
option-line-dashed = Штрих

plot-type-frequency = АЧХ
plot-type-phase = ФЧХ
plot-type-impulse = ИХ
plot-type-step = Переходная характеристика
plot-type-operation = Выходной сигнал

line-design-frequency = АЧХ
line-design-phase = ФЧХ
line-design-impulse = ИХ
line-design-step = Переходная характеристика
line-operation-input = Вход. сигнал
line-operation-output = Вых. сигнал
line-operation-corrected = Скоррект. сигнал
line-operation-reference = Эталон. сигнал

table-type-frequency = Частотные характеристики
table-type-impulse = Импульсная характеристика
table-type-step = Переходная характеристика

button-save-csv = Сохранить CSV
button-save-plot = Сохранить график PNG
button-compute = Вычислить
button-reset-params = Сбросить параметры
button-copy = Коп.
button-cancel = Отмена
button-ok = Ок

checkbox-auto-compute = Автовычисление

color-blue = Синий
color-orange = Оранжевый
color-green = Зеленый
color-red = Красный
color-cyan = Голубой

plot-tooltip-zoom-out = Увеличить область отображения.
plot-tooltip-fit = Выполнить авто-масштабирование по текущим данным.
plot-tooltip-zoom-in = Уменьшить область отображения.

x-axis-frequency = Частота ω, рад/сек
x-axis-time = Время t, с
unit-seconds =  сек
unit-rad-sec = рад/сек
unit-px = px

msg-no-results = Чтобы отобразить данные, программа сначала должна произвести вычисления.
msg-about-authors = Авторы:
msg-about-description = Filtr — интерактивный инструмент ЦОС для проектирования фильтров, обработки сигналов и визуализации характеристик.
msg-about-version = Версия:
about-author-1 = > Александр Бегичев
about-author-2 = > Владислав Жижин
about-author-3 = > Григорий Чикильдин

msg-reset-confirm-title = Подтверждение
msg-reset-confirm-body = Сбросить все параметры к значениям по умолчанию?
msg-notice-title = Уведомление

msg-error-prefix = Ошибка:
msg-compute-auto-disabled = { $error } Автовычисление отключено. Исправьте параметры и нажмите "{ button-compute }".
msg-compute-failed-auto-disabled = Ошибка вычисления: параметры выходят за допустимые пределы. Автовычисление отключено.

err-dt-positive = Параметр Δt должен быть больше 0.
err-dw-positive = Параметр Δω должен быть больше 0.
err-nan-inf = Некорректные значения параметров (NaN/Inf).
err-v1 = Параметр V* должен быть больше 1.
err-delta-range = Параметр δ должен быть в диапазоне (0, 1).
err-big-delta-range = Параметр Δ должен быть в диапазоне (0, 1).
err-design-freq-order = Частоты проектирования должны удовлетворять: 0 ≤ ωs ≤ Ωφ.
err-kf-overflow = Слишком мелкий шаг Δω или большая Ωφ: индекс частоты { $kf } превышает { $max }.
err-nt-overflow = Слишком большая длительность переходного процесса: Nt={ $nt} превышает { $max }.
err-ne-overflow = Слишком большая длительность сигнала: Ne={ $ne } превышает { $max }.
err-nx-lto-overflow = Слишком большие L/To: Nx={ $nx } превышает { $max }.
err-nx-tptf-overflow = Слишком большие Tp/Tf/To: Nx={ $nx } превышает { $max }.

table-col-frequency = Частота
table-col-magnitude = АЧХ
table-col-phase = ФЧХ
csv-header-frequency = N,Частота,АЧХ,ФЧХ

msg-params-saved = Параметры сохранены: { $path }
msg-params-loaded = Параметры загружены: { $path }
err-params-serialize = не удалось сериализовать параметры: { $error }
err-params-decode = не удалось декодировать RON: { $error }
err-params-write = не удалось записать файл { $path }: { $error }
err-params-save = Ошибка: не удалось сохранить параметры: { $error }
err-params-read = не удалось прочитать файл { $path }: { $error }
err-params-invalid-ron = некорректный RON в { $path }: { $error }
err-params-encoding = некорректная кодировка файла: { $error }

msg-csv-saved = CSV сохранен: { $path }
msg-csv-saved-short = CSV сохранен.
msg-csv-cancelled = Сохранение CSV отменено.
err-csv-write = Ошибка: не удалось записать CSV { $path }: { $error }
err-csv-write-short = Ошибка: не удалось записать CSV: { $error }

err-png-prepare = не удалось подготовить PNG: { $error }
err-plot-region = не удалось определить область графика. Сначала откройте график на экране.
msg-png-cancelled = Сохранение PNG отменено.
err-png-save = не удалось сохранить PNG { $path }: { $error }
msg-plot-saved = График сохранен: { $path }
err-plot-region-short = Ошибка: не удалось определить область графика.
msg-plot-saved-short = График сохранен.
err-png-save-short = Ошибка: не удалось сохранить PNG: { $error }

filter-mode-tooltip = Режим работы программы.
filter-type-tooltip = Вид фильтра: нерекурсивный или рекурсивный.
filter-category-tooltip = Тип фильтра: НЧФ, ПФ, РФ, ВЧФ или ДФ.
dt-tooltip = Шаг дискретизации по времени.

passband-ripple-tooltip = Неравномерность АЧХ фильтра в полосе пропускания.
stopband-ripple-tooltip = Неравномерность АЧХ фильтра в полосе задерживания.
transition-band-tooltip = Параметр, определяющий ширину переходных полос АЧХ.
gain-tooltip = Коэффициент усиления в полосе пропускания АЧХ фильтра.

cutoff-lower-tooltip = Нижняя граничная частота полосы пропускания АЧХ фильтра.
cutoff-upper-tooltip = Верхняя граничная частота полосы пропускания АЧХ фильтра.

task-tooltip = Выбор задачи обработки сигнала.
main-interval-tooltip = Длительность основного интервала.
impulse-len-tooltip = Длина импульсной характеристики.
effective-len-tooltip = Эффективная длительность временной характеристики.
phase-delay-tooltip = Фазовая задержка.
signal-tooltip = Выбор входного сигнала.
noise-level-tooltip = Уровень помехи.
noise-low-tooltip = Нижняя частота эффективной полосы частотного спектра помехи.
noise-high-tooltip = Верхняя частота эффективной полосы частотного спектра помехи.
noise-transition-tooltip = Ширина переходной полосы частотного спектра помехи.

recursive-level-tooltip = Уровень, определяющий эффективную длительность временной характеристики.
recursive-duration-tooltip = Длительность определения временной характеристики.

dw-tooltip = Шаг дискретизации по частоте.
design-low-tooltip = Нижняя частота диапазона определения АЧХ и ФЧХ.
design-high-tooltip = Верхняя частота диапазона определения АЧХ и ФЧХ.
copy-tooltip = Скопировать значение в буфер обмена.
