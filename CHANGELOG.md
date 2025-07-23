## [0.0.21] - Unreleased

### Changed

- Silence warnings


## [0.0.20] - 2025-07-23

### Added

- Add reset button

### Changed

- Round Niseci, RQE Niseci, Hfbi to 3 decimal digits
- Drop empty lines in log files
- Invert page 1 and 2 for Niseci PDF

## [0.0.19] - 2025-07-21

### Added

- Collect `SubmetricheX3`
- Add condizioni riferimento HFBI
- Add full hfbi calc
- Add `esporta_pdf_hfbi()`
- Add tests for hfbi modules

### Changed

- Log codice stazione
- Use `String::with_capacity()` for textbox buffers
- Fix: always drop index 1 for mutargs in `run_headless()`
- Fix wrong include using old name for `csv::deser`
- Fix 2 failing tests by adjusting expected values

## [0.0.18] - 2025-06-21

### Added

- New `pdf` module with `esporta_pdf_niseci()`
- New `hfbi` module in `domain`
- New functions related to parsing `CampionamentoHFBI` from csv
- All code paths up to HFBI calc request have been implemented
- Added `templates/campionamento_hfbi.csv`

## [0.0.17] - 2025-06-17

## Changed

- Fix error in x3 calc

## [0.0.16] - 2025-06-13

### Changed

- Split log in two files
- Fix error in x1 calc
- Fix wrong text spacing in info box

## [0.0.15] - 2025-06-03

### Changed

- Use git reference for raylib
- Add own definition for rrect() since it was dropped from raylib crate
- Refactor Display for domain::niseci to allow a csv-like output
- Refactor textbox components in views module to avoid using null-terminated strings
- Changed window title to F-value

## [0.0.14] - 2025-05-29

### Changed

- Bump `raylib` to `5.5.1`
- Avoid using `gui_panel()` until `5.5.x` behaves like `5.0.x`
- Rename `core::csv::lexer` to `core::csv::deser`

## [0.0.13] - 2025-04-12

### Added

- Add `assets/FreeMono.ttf` and use it instead of `assets/ubuntu.mono.ttf`
- New traits:
  - `Controller`
  - `View`
  - `SubModel`
- New structs:
  - `Controllers`
  - `Views`
  - `RecordSubmetricheX2A`
    - Used to relax the last tuple non-type left after the intermediates refactor: `(String, MetricheX2A, ClassiEtaSpecieNISECI)`
- Impl `TryFrom<i32>` for:
  - `TipoComunitaNISECI`
  - `AreaNISECI`
  - `IdroEcoRegioneNISECI`
- New `logged` feature, enabling `prep_logger()`
  - Uses `dirs` and `flexi_logger` crates

### Changed
- Refactor `RecordCsvRiferimentoNISECI`, `RecordCsvCampionamentoNISECI`, `RecordCsvAnagraficaNISECI` structs to be traits
  - All methods involving them are now generic
  - Previous struct are renamed to:
    - `VeryItalianRecordCsvRiferimentoNISECI`
    - `VeryItalianRecordCsvCampionamentoNISECI`
    - `VeryItalianRecordCsvAnagraficaNISECI`
  - New struct without custom deserializers:
    - `PlainRecordCsvRiferimentoNISECI`
    - `PlainRecordCsvCampionamentoNISECI`
    - `PlainRecordCsvAnagraficaNISECI`
  - Keeps current behaviour
- Fix: avoid double check on `tipo_autoctono == 1` in `CampionamentoNISECI::get_tot_specie_autoctone()`
- Solve most `clippy` hints
- Set `Console.columns` to `65` rather than `80` for new font, to keep the look
- Mark all previously `pub` APIs as `pub(crate)`
- Refactor `core` module:
  - Move gui-related code to `app` module
  - Move parser-related code to `core::csv` module
    - Deserialize step in `core::csv::lexer`
    - Value-check step in `core::csv::parser`
- Refactor `model` module:
  - Moved domain model to `domain`
  - Moved gui model to `app::model`
- Clean up star imports
- Use double text spacing for `Light` theme
- Empty string fields in `SelezioneInfoAggiuntiveView`

## [0.0.12] - 2025-03-17

### Added
- Add `assets/logo.png`
- Embedding logo increases binary size:
  - Target `x86_64-pc-windows-gnu`:
    - Absolute: +1.03MB
    - Relative: +25.6%
- Add missing tests for anagrafica NISECI csv parsing
- Add license info button
- Add backout button in `ConsoleView`

### Changed

- Refactor `RecordCsvCampionamentoNISECI`:
  - Drop field `superficie`
  - Refactor field `numPassaggio` to be `u32`
  - Refactor fields `lunghezza`, `peso` to be `u32`
- Refactor `RecordCsvAnagraficaNISECI`:
  - Refactor fields `larghezza_stazione`, `lunghezza_stazione` to be `f32`
  - Refactor `SelezioneInfoAggiuntiveView` to handle the check
    - ATM it's done by `InfoAggiuntiveController::check_larghezza_stazione_str()`
    - Also updates the model and sets the error flag, going to `ConsoleView`
- Refactor `RecordCsvRiferimentoNISECI`
    - Refactor fields `tipo_autoctono`, `allo_nocivita`, `specie_attesa` to be `u32`
- Flag `--info` now also prints expected types
- Fixed wrong message in error reports for `check_riferimento_niseci_reader`
- Refactor theme loading into `core::controller::GuiTheme::load_and_set()`
- Use double default font height on `GuiTheme::Light`
  - At startup (`MainState`, `ConsoleView` prep)
  - On theme switch
- Replace `razionale` with `decimale` in hints
- Ensure to set default font height on theme change
- Tidy up some views to handle the bigger default text
- Refactor `MainState.previous_view` to drop the `Option` wrap
- Refactor `Console::draw()` to pass in `ConsoleController`
  - Only used for backout button logic

## [0.0.11] - 2025-03-12

### Added

- More info on HOME and SECOND view
- Store `RisultatoNISECI` as console env, for info command
- Display `x1`, `x2`, `x3` on `ProduzioneOutputView`
- Added format functions for some model structs
- New types for holding intermediate values in `RisultatoNISECI`:
  - `ValoriIntermediNISECI`
  - `ValoriIntermediSpecieNISECI`
- New utility types for low-level calls:
  - `MetricheX2`
  - `SubmetricheX2`
    - `MetricheX2A`
    - `MetricheX2aB`
    - `MetricheX2B`
  - `MetricheX3`
- `HomeView`, `SecondView` improvements
  - Added copyright info
  - Added continue button

### Changed

- Fixed broken author link
- Improved console backout hint
- Refactored engines to return niseci intermediate values
    - Made x2 and full result optional values
- `RisultatoNISECI` now holds intermediates
- Fix `calculate_x2()` divide-by-zero
- Update tests
- Less verbose CLI output
- Print intermediate values to stdout and console on full calc
- Use spinner for lunghezza, larghezza in `InfoAggiuntiveView`
- Updated copyright info splash

## [0.0.10] - 2025-03-04

### Added

- Calc x2
- Calc x3
- New utility structs for engines
  - `ClassiEtaSpecieNISECI`
  - `ClassiEta`
  - `InfoPopolazioniNISECI`
  - `InfoPopolazioniAlieneNISECI`
  - `ClassiEtaAlieniNISECI`
  - `EsemplariPerCattura`
- More tests
  - `x2` (also private methods tests)
  - `x3`
  - `niseci::full` (minimal)
- Add `templates/riferimento_niseci.csv`
- Add list_view for `IdroEcoRegioneNISECI` in `SelezionaInfoAggiuntiveView`
- Add `AreaNISECI`
- Add `console.unset_env()` to drop anagrafica_niseci on user backout
- Add submit for `Anagrafica`
  - Ensures no stray nullbytes in strings
- Adds heading on all source files with copyright info
- Adds `GPL-3.0-only` license
- `InfoAggiuntiveView` collects `date_string`
  - Previously, the text buffer contents were unused
  - Format expected: `dd/mm/yyyy`
- `InfoAggiuntiveController` handles `AnagraficaNISECI` validation
- `OutputController` goes to `Console` on errors
- Add `calculate_niseci()`
- Add `calculate_stato_ecologico()`
- Add `calculate_rqe_niseci()`
- Add parsing funcs for anagrafica niseci csv
- Check anagrafica file in `run_headless()`
- Add full niseci calc in `run_headless()`
- Add `templates/anagrafica_niseci.csv` to support full headless run
- Store niseci result, display it in view
  - Adds handling of user confirm in `ProduzioneOutput` view to show output
there
- Add `CHANGELOG.md`
- Adds a note in `ConsoleView` suggesting users to click up to return
- Print splash on stdout
- Handle `-W` to print warranty notice
- Update infobox
  - Add link to spdx for gpl-3.0-only
  - Add author github links
- Add `README.md`
- Add `NormalizerReader` to handle accented vowels

### Changed

* `RecordCsvRiferimentoNISECI` ora ha i campi `cl_soglia_N` dove `N = 1, 2, 3 , 4`
- Refactor `InfoAggiuntiveView`
- Add list_view for `regione` in `SelezionaInfoAggiuntiveView`
- Renamed `templates/campionamento_NISECI.csv` to `templates/campionamento_niseci.csv`
- Refactor `codice_stazione` handling
- Rename field `nome_fiume` to `corpo_idrico` in `AnagraficaNISECI`
- Minor refactor to resolve all warnings
- Some functions/includes which were only needed for test builds are not correctly marked with `#[cfg(test)]`
- Updated float comparisons to always use an inlined epsilon
- Use crate uuid to randomise tempfile names
- Accept `.CSV` files
- Fix empty error when filepicker gets a non-csv file

## [0.0.9] - 2025-01-23

### Added

- Add `Console`, `ConsoleView`, `ConsoleController`
  - Featuring user prompt with commands
  - Scrollable with both mouse and arrow keys
  - Uses monospaced font
- Add `deserialize_comma_f32()` to give very italian float literals a chance
- Improved `process_csv_errors()` handling of field position for deserialize errors
- Console messages for errors
- Improved `FileInputController::{valida_riferimento_niseci_path, valida_campionamento_niseci_path}`
  - Switch to console view on errors
  - Value-check step after previous csv-check
  - Switch to `SelezioneInfoAggiuntive` view on successful validation
- New `data_model` field in `Model` to hold the validated values after processing for actual usage in calcs
  - Stores `RiferimentoNISECI`, `CampionamentoNISECI`
- Add funcs for linear regression and calcolo quantita stimata
- Add `calculate_x1`

### Changed

- Handle `c3`, `c4` in `parse_recordcsv_campionamento_niseci()`
- Refactor test modulari

## [0.0.8] - 2024-12-19

### Added

- Value-check functions for NISECI:
  - `check_records_riferimento_niseci()`, to value-check `Vec<RecordCsvRiferimentoNISECI>` (to be improved)
  - `check_records_campionamento_niseci()` to value-check `(Vec<RecordCsvCampionamentoNISECI>, Vec<SpecieNISECI>)` (to be improved)
  - Test for valid cases for both  of them
- Handle value-check step in `run_headless()`
- Added statusbar, removed text drawing for current view
  - The previous text is now used for the statusbar
- Added navbar with panels to group controls
- Added icontext in `SelezioneFileInputView`
- Handle clear selection in `SelezioneFileInputView`

### Changed
- Moved core buttons to navbar
- Fix: Handle locking view when a window_box is shown from `draw_main()`
  - Fixed gui controls working behind window_box
  - All views now correctly lock as expected, behind the current
    windowbox from `draw_main()`
- `ValidazioneFileInputView` disattiva tasto campionamento quando opportuno

## [0.0.7] - 2024-12-16

### Added

- New method for `FileInputController`
  - `valida_{riferimento,campionamento}_niseci_path()`
  - For now, they only call the csv check. Soon they'll also call the values check
- Select input file using `rfd`
- New Pathbuf fields in `fileinput_model`

## [0.0.6] - 2024-12-14

### Added

- Selecting an index changes view
- Domain module

### Changed

- `SelezionaFileInputView` goes to `exit()` if `indice_model.get_selected_index()` is `None`

## [0.0.5] - 2024-12-14

### Added

- Add tests for csv check functions
- Improved output format
- Template `campionamento_niseci.csv`
- Handle `--info` to print expected csv headers
- New tests module
- Translation step for error messages

### Changed

- Refactor check functions to take a Read

## [0.0.4] - 2024-12-12

### Added

- Parse csv step functions
  - Expects ";" delimiter
- Expect camelCase for actual csv header in the input

### Changed

- Fix `gui_dropdown_box()` being drawn too early

## [0.0.3] - 2024-12-12

### Added

- Base for all views
- Handle `--headless`, `--help`
- Improved infobox

### Changed

- Use `font.measure_text()` instead of `d.measure_text()` in `draw_todo_view_text()`

## [0.0.2] - 2024-12-09

### Added

- Basic views
- Embed styles
  - Must ensure `GUI_THEME_COMBOBOX_STR` respect the order of discriminants in `GuiTheme`

## [0.0.1] - 2024-11-21

### Added

- First commit
