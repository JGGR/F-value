## [0.0.10] - Unreleased

### Added

- Calc x2
- Calc x3
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
  - Closes #7
- Add `AreaNISECI`
  - Closes #8
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
- Add `calculate_stato_ecologico()`, use `log(10.0)` for rqe_niseci
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

### Changed

* `RecordCsvRiferimentoNISECI` ora ha i campi `cl_soglia_N` dove `N = 1, 2, 3 , 4`
- Refactor `InfoAggiuntiveView`
- Add list_view for `regione` in `SelezionaInfoAggiuntiveView`
- Renamed `templates/campionamento_NISECI.csv` to `templates/campionamento_niseci.csv`
- Refactor `codice_stazione` handling
- Rename field `nome_fiume` to `corpo_idrico` in `AnagraficaNISECI`
- Minor refactor to resolve all warnings
- Some functions/includes which were only needed for test builds are not correctly marked with `#[cfg(test)]`

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
