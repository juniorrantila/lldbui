use std::fs::read_to_string;
use std::path::PathBuf;

use egui::{Align, Rect, RichText, ScrollArea, Ui};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use lldb::SBCompileUnit;

use crate::app::widgets::{AnsiString, IconArrow, IconBreakpoint};
use crate::app::App;
use crate::debugger;

pub fn add(app: &mut App, ui: &mut Ui) {
    let frame = app.target.process().selected_thread().selected_frame();

    if let Some(line_entry) = frame.line_entry() {
        let scroll = line_entry.filespec().filename() != app.source_file
            || line_entry.line() != app.source_line;
        app.source_file = line_entry.filespec().filename().to_string();
        app.source_line = line_entry.line();

        let path: PathBuf = [
            line_entry.filespec().directory(),
            line_entry.filespec().filename(),
        ]
        .iter()
        .collect();

        let key = path.clone().into_os_string().into_string().unwrap();
        ui.label(&key);
        ui.separator();

        if path.exists() {
            let source = app
                .source_cache
                .entry(path.to_str().unwrap().to_string())
                .or_insert(read_to_string(path).unwrap());
            let theme = &CodeTheme::from_style(ui.style());
            let language = detect_language(frame.compile_unit());

            let row_height = ui.spacing().interact_size.y;
            let total_rows = source.lines().count();
            let target_line = line_entry.line() as usize;

            ScrollArea::both()
                .auto_shrink(false)
                .animated(false)
                .show_rows(ui, row_height, total_rows, |ui, mut row_range| {
                    let first = row_range.next().unwrap_or(0);
                    let last = row_range.next_back().unwrap_or(first);
                    let mut scroll_source_rect = Rect::NOTHING;
                    egui::Grid::new("source")
                        .num_columns(4)
                        .min_col_width(5.0)
                        .start_row(first)
                        .with_row_color(move |i, style| {
                            if i + 1 == target_line {
                                Some(style.visuals.faint_bg_color)
                            } else {
                                None
                            }
                        })
                        .show(ui, |ui| {
                            let mut i = first;
                            for line in source.lines().skip(i).take((last - first) + 1) {
                                i += 1;
                                let mut breakpoint = None;
                                for (bp_id, bp_file, bp_line) in
                                    debugger::breakpoint_locations(&app.target).iter()
                                {
                                    if line_entry.filespec().filename() == bp_file
                                        && i == *bp_line as usize
                                    {
                                        breakpoint = Some(*bp_id);
                                        break;
                                    }
                                }
                                let hover_text = if breakpoint.is_some() {
                                    "delete"
                                } else {
                                    "add breakpoint"
                                };
                                if ui
                                    .add(IconBreakpoint::new(breakpoint.is_some()))
                                    .on_hover_text(hover_text)
                                    .clicked()
                                {
                                    if let Some(bp_id) = breakpoint {
                                        app.target.delete_breakpoint(bp_id);
                                    } else {
                                        let bp = app.target.breakpoint_create_by_location(
                                            line_entry.filespec().filename(),
                                            i as u32,
                                        );
                                        tracing::debug!("breakpoint created: {:?}", bp);
                                    }
                                };

                                if i == target_line {
                                    ui.add(IconArrow::new(ui.style().visuals.warn_fg_color));
                                } else {
                                    ui.label(" ");
                                }

                                let mut line_number = RichText::new(format!("{}", i));
                                if i == target_line {
                                    line_number =
                                        line_number.color(ui.style().visuals.warn_fg_color);
                                }
                                ui.label(line_number);
                                let layout_job = highlight(ui.ctx(), theme, line, &language);
                                let response =
                                    ui.add(egui::Label::new(layout_job).selectable(true));
                                // record location of first line to later calculate a scroll offset
                                if scroll && i - 1 == first {
                                    scroll_source_rect = response.rect;
                                }
                                ui.end_row();
                            }
                        });
                    // scroll to the target line
                    if scroll {
                        let line_diff = target_line as i32 - (first + 1) as i32;
                        let spacing_y = ui.spacing().item_spacing.y;
                        let y_diff = line_diff as f32 * (row_height + spacing_y);
                        scroll_source_rect.min.y += y_diff;
                        scroll_source_rect.max.y += y_diff;
                        ui.scroll_to_rect(scroll_source_rect, Some(Align::Center));
                    };
                });
        } else {
            tracing::info!("source file not found: {}", path.display());
        }
    } else {
        // show disassembly
        let function = frame.function();
        if function.is_valid() {
            ui.label(function.display_name());
            ui.separator();
        }
        let symbol = frame.symbol();
        if symbol.is_valid() {
            ui.label(symbol.display_name());
            ui.separator();
        }
        ScrollArea::both()
            .auto_shrink(false)
            .animated(false)
            .show(ui, |ui| {
                ui.add(AnsiString::new(frame.disassemble()));
            });
    }
}

// https://github.com/trishume/syntect
// Supported file types:
// - Plain Text (.txt)
// - ASP (.asa)
// - HTML (ASP) (.asp)
// - ActionScript (.as)
// - AppleScript (.applescript, .script editor)
// - Batch File (.bat, .cmd)
// - NAnt Build File (.build)
// - C# (.cs, .csx)
// - C++ (.cpp, .cc, .cp, .cxx, .c++, .C, .h, .hh, .hpp, .hxx, .h++, .inl, .ipp)
// - C (.c, .h)
// - CSS (.css, .css.erb, .css.liquid)
// - Clojure (.clj)
// - D (.d, .di)
// - Diff (.diff, .patch)
// - Erlang (.erl, .hrl, .Emakefile, .emakefile)
// - HTML (Erlang) (.yaws)
// - Go (.go)
// - Graphviz (DOT) (.dot, .DOT, .gv)
// - Groovy (.groovy, .gvy, .gradle)
// - HTML (.html, .htm, .shtml, .xhtml, .inc, .tmpl, .tpl)
// - Haskell (.hs)
// - Literate Haskell (.lhs)
// - Java Server Page (JSP) (.jsp)
// - Java (.java, .bsh)
// - JavaDoc (.)
// - Java Properties (.properties)
// - JSON (.json, .sublime-settings, .sublime-menu, .sublime-keymap, .sublime-mousemap, .sublime-theme, .sublime-build, .sublime-project, .sublime-completions, .sublime-commands, .sublime-macro, .sublime-color-scheme)
// - JavaScript (.js, .htc)
// - Regular Expressions (Javascript) (.)
// - BibTeX (.bib)
// - LaTeX Log (.)
// - LaTeX (.tex, .ltx)
// - TeX (.sty, .cls)
// - Lisp (.lisp, .cl, .clisp, .l, .mud, .el, .scm, .ss, .lsp, .fasl)
// - Lua (.lua)
// - Make Output (.)
// - Makefile (.make, .GNUmakefile, .makefile, .Makefile, .OCamlMakefile, .mak, .mk)
// - Markdown (.md, .mdown, .markdown, .markdn)
// - MultiMarkdown (.)
// - MATLAB (.matlab)
// - OCaml (.ml, .mli)
// - OCamllex (.mll)
// - OCamlyacc (.mly)
// - camlp4 (.)
// - Objective-C++ (.mm, .M, .h)
// - Objective-C (.m, .h)
// - PHP Source (.)
// - PHP (.php, .php3, .php4, .php5, .php7, .phps, .phpt, .phtml)
// - Pascal (.pas, .p, .dpr)
// - Perl (.pl, .pm, .pod, .t, .PL)
// - Python (.py, .py3, .pyw, .pyi, .pyx, .pyx.in, .pxd, .pxd.in, .pxi, .pxi.in, .rpy, .cpy, .SConstruct, .Sconstruct, .sconstruct, .SConscript, .gyp, .gypi, .Snakefile, .wscript)
// - Regular Expressions (Python) (.)
// - R Console (.)
// - R (.R, .r, .s, .S, .Rprofile)
// - Rd (R Documentation) (.rd)
// - HTML (Rails) (.rails, .rhtml, .erb, .html.erb)
// - JavaScript (Rails) (.js.erb)
// - Ruby Haml (.haml, .sass)
// - Ruby on Rails (.rxml, .builder)
// - SQL (Rails) (.erbsql, .sql.erb)
// - Regular Expression (.re)
// - reStructuredText (.rst, .rest)
// - Ruby (.rb, .Appfile, .Appraisals, .Berksfile, .Brewfile, .capfile, .cgi, .Cheffile, .config.ru, .Deliverfile, .Fastfile, .fcgi, .Gemfile, .gemspec, .Guardfile, .irbrc, .jbuilder, .podspec, .prawn, .rabl, .rake, .Rakefile, .Rantfile, .rbx, .rjs, .ruby.rail, .Scanfile, .simplecov, .Snapfile, .thor, .Thorfile, .Vagrantfile)
// - Cargo Build Results (.)
// - Rust (.rs)
// - SQL (.sql, .ddl, .dml)
// - Scala (.scala, .sbt)
// - Bourne Again Shell (bash) (.sh, .bash, .zsh, .fish, ..bash_aliases, ..bash_completions, ..bash_functions, ..bash_login, ..bash_logout, ..bash_profile, ..bash_variables, ..bashrc, ..profile, ..textmate_init)
// - Shell-Unix-Generic (.)
// - commands-builtin-shell-bash (.)
// - HTML (Tcl) (.adp)
// - Tcl (.tcl)
// - Textile (.textile)
// - XML (.xml, .xsd, .xslt, .tld, .dtml, .rss, .opml, .svg)
// - YAML (.yaml, .yml, .sublime-syntax)
fn detect_language(compile_unit: SBCompileUnit) -> String {
    let str = match compile_unit.language() {
        lldb::LanguageType::Unknown => todo!(),
        lldb::LanguageType::C89 => "C",
        lldb::LanguageType::C => "C",
        lldb::LanguageType::Ada83 => todo!(),
        lldb::LanguageType::C_plus_plus => "C++",
        lldb::LanguageType::Cobol74 => todo!(),
        lldb::LanguageType::Cobol85 => todo!(),
        lldb::LanguageType::Fortran77 => todo!(),
        lldb::LanguageType::Fortran90 => todo!(),
        lldb::LanguageType::Pascal83 => todo!(),
        lldb::LanguageType::Modula2 => todo!(),
        lldb::LanguageType::Java => "Java",
        lldb::LanguageType::C99 => "C",
        lldb::LanguageType::Ada95 => todo!(),
        lldb::LanguageType::Fortran95 => todo!(),
        lldb::LanguageType::PLI => todo!(),
        lldb::LanguageType::ObjC => "Objective-C",
        lldb::LanguageType::ObjC_plus_plus => "Objective-C++",
        lldb::LanguageType::UPC => todo!(),
        lldb::LanguageType::D => "D",
        lldb::LanguageType::Python => todo!(),
        lldb::LanguageType::OpenCL => todo!(),
        lldb::LanguageType::Go => "Go",
        lldb::LanguageType::Modula3 => todo!(),
        lldb::LanguageType::Haskell => todo!(),
        lldb::LanguageType::C_plus_plus_03 => "C++",
        lldb::LanguageType::C_plus_plus_11 => "C++",
        lldb::LanguageType::OCaml => "OCaml",
        lldb::LanguageType::Rust => "Rust",
        lldb::LanguageType::C11 => "C",
        lldb::LanguageType::Swift => todo!(),
        lldb::LanguageType::Julia => todo!(),
        lldb::LanguageType::Dylan => todo!(),
        lldb::LanguageType::C_plus_plus_14 => "C++",
        lldb::LanguageType::Fortran03 => todo!(),
        lldb::LanguageType::Fortran08 => todo!(),
        lldb::LanguageType::MipsAssembler => todo!(),
        lldb::LanguageType::ExtRenderScript => todo!(),
        lldb::LanguageType::NumLanguageTypes => todo!(),
    };
    str.to_string()
}
