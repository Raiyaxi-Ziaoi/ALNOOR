use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn main() -> io::Result<()> {
    let command_line_arguments: Vec<String> = env::args().collect(); // COMMAND LINE ARGUMENTS
    let cleanup_mode: &String = &command_line_arguments[1];

    let mut skip: bool = false;

    // OTHER ARGS {

    if cleanup_mode == "-v" || cleanup_mode == "--version" {
        println!(
            "{}",
            read_to_string("STD/CORE/.version").expect("Version Missing")
        );
        std::process::abort();
    } else if cleanup_mode == "-h" || cleanup_mode == "--help" || cleanup_mode == "-?" {
        println!("(./)lnrc(.exe) [Cleanup mode] [File path] [Config file path]\nCleanup modes:\n-a : All\n-m : Manifest\n-j : .java\n-c : .class\n-n : None\nAdd -cmd to clean up shell file");
        std::process::abort();
    }

    let source_file: &String = &command_line_arguments[2];

    if cleanup_mode == "-innit" || cleanup_mode == "-init" || cleanup_mode == "-i" {
        fs::create_dir(source_file)?;

        let formatted_config_path = format!("{}/config.vn", source_file);

        // CONFIG {
        let path_config: &Path = Path::new(&formatted_config_path);
        let display_config = path_config.display();

        let _file_config: File = match File::create(format!("{}/config.vn", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_config, why),
            Ok(file_config) => file_config,
        };

        // }

        // SOURCE {

        let formatted_source_path = format!("{}/main.lsmx", source_file);

        let path_source: &Path = Path::new(&formatted_source_path);
        let display_source = path_source.display();

        let _file_source: File = match File::create(format!("{}/main.lsmx", source_file)) {
            Err(why) => panic!("Could not create {}: {}", display_source, why),
            Ok(file_source) => file_source,
        };

        std::process::abort();
        // }
    }

    // }

    let import_path: &String = &command_line_arguments[3];

    // IMPORTS {
    if import_path == "None" {
        skip = true;
    }
    let config_path: &Path = Path::new(import_path);
    let splitted_config: Vec<&str> = import_path.split(".").collect();

    if splitted_config[1] != "vn" {
        panic!("Wrong filetype! Please ensure that the file ends with \".vn\"");
    }

    let display_config = config_path.display();
    let mut config_file: File = match File::open(&config_path) {
        Err(why) => panic!("Could not open {}: {}", display_config, why),
        Ok(config_file) => config_file,
    };

    let mut config_file_contents: String = String::new();
    match config_file.read_to_string(&mut config_file_contents) {
        Err(why) => panic!("Could not read {}: {}", display_config, why),
        Ok(_) => println!(""),
    }

    let to_import = file_to_vec(import_path.to_string())?;

    // }

    let source_path: &Path = Path::new(source_file);

    // FILE {

    let display_source = source_path.display();
    let mut file_source: File = match File::open(&source_path) {
        Err(why) => panic!("Could not open {}: {}", display_source, why),
        Ok(file_source) => file_source,
    };

    let mut file_contents: String = String::new();
    match file_source.read_to_string(&mut file_contents) {
        Err(why) => panic!("Could not read {}: {}", display_source, why),
        Ok(_) => println!(""),
    }

    // }

    // FILEEXT & FILENAME

    let splitted_name: Vec<&str> = source_file.split(".").collect();
    let endf: Vec<&str> = splitted_name[0].split("/").collect();
    let namef = endf[endf.len() - 1];

    let javaf: String = format!("{}.java", namef);
    let classf: String = format!("{}.class", namef);
    let jarf: String = format!("{}.jar", namef);

    if splitted_name[1] != "lsmx" {
        panic!("Wrong filetype! Please ensure that the file ends with \".lsmx\"");
    }

    // MANIFEST {

    let path_manifest: &Path = Path::new("Manifest.txt");
    let display_manifest = path_manifest.display();

    let mut file_manifest: File = match File::create("Manifest.txt") {
        Err(why) => panic!("Could not create {}: {}", display_manifest, why),
        Ok(file_manifest) => file_manifest,
    };

    let manifest: String = format!("Main-Class: {}", namef);
    match file_manifest.write_all(manifest.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display_manifest, why),
        Ok(_) => println!("Successfully wrote to {}", display_manifest),
    }

    // }

    // JAVA {

    let path_java: &Path = Path::new(&javaf);
    let display_java = path_java.display();

    let mut java_write: File = match File::create(&path_java) {
        Err(why) => panic!("Could not create {}: {}", display_java, why),
        Ok(java_write) => java_write,
    };

    // }

    // IMPORTS {

    let mut imported: String = "".to_owned();
    let mut aditlibs: String = "".to_owned();
    let mut libs: String = "".to_owned();
    let mut ext: String = "".to_owned();

    let mut usescan: bool = false;

    if !skip {
        for import in to_import {
            if import.len() > 6 {
                if import == "None" {
                    break;
                } else if import == "STD.IO.STR" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_str.ryx")
                        .expect("STD.IO.STR Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.INT" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_int.ryx")
                        .expect("STD.IO.INT Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.FLOAT" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_flo.ryx")
                        .expect("STD.IO.FLOAT Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.BYTE" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_byt.ryx")
                        .expect("STD.IO.BYTE Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.SHORT" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_sho.ryx")
                        .expect("STD.IO.SHORT Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.BOOL" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_boo.ryx")
                        .expect("STD.IO.BOOL Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.CHAR" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_cha.ryx")
                        .expect("STD.IO.CHAR Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.DOUBLE" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_dou.ryx")
                        .expect("STD.IO.DOUBLE Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.IO.LONG" {
                    let inlib: String = read_to_string("STD/ALNOOR_J/IO/in_lon.ryx")
                        .expect("STD.IO.LONG Library Missing");
                    aditlibs.push_str(&inlib);
                    usescan = true;
                } else if import == "STD.STAT.CASE" {
                    let caselib: String = read_to_string("STD/ALNOOR_J/STAT/case.ryx")
                        .expect("STD.STAT.CASE Library Missing");
                    aditlibs.push_str(&caselib);
                } else if import == "STD.FUNC.SLEEP" {
                    let waitlib: String = read_to_string("STD/ALNOOR_J/FUNC/sleep.ryx")
                        .expect("STD.FUNC.SLEEP Library Missing");
                    aditlibs.push_str(&waitlib);
                } else if import == "STD.STAT.DATETIME" {
                    let datetimelib: String = read_to_string("STD/ALNOOR_J/STAT/datetime.ryx")
                        .expect("STD.STAT.DATETIME Library Missing");
                    aditlibs.push_str(&datetimelib);
                    let str: String = "\nimport java.text.DateFormat;\nimport java.text.ParseException;\nimport java.text.SimpleDateFormat;\nimport java.util.Date;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.IO.ARRAYS" {
                    let outlib: String = read_to_string("STD/ALNOOR_J/IO/arrays.ryx")
                        .expect("STD.IO.ARRAYS Library Missing");
                    aditlibs.push_str(&outlib);
                } else if import == "STD.STAT.REGEX" {
                    let rgxlib: String = read_to_string("STD/ALNOOR_J/STAT/regex.ryx")
                        .expect("STD.STAT.REGEX Library Missing");
                    aditlibs.push_str(&rgxlib);
                    let str: String =
                        "\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;"
                            .to_string();
                    imported.push_str(&str);
                } else if import == "STD.STAT.SORT" {
                    let srtlib: String = read_to_string("STD/ALNOOR_J/STAT/sort.ryx")
                        .expect("STD.STAT.SORT Library Missing");
                    aditlibs.push_str(&srtlib);
                } else if import == "STD.FUNC.SHELL" {
                    let shelllib: String = read_to_string("STD/ALNOOR_J/FUNC/shell.ryx")
                        .expect("STD.FUNC.SHELL Library Missing");
                    aditlibs.push_str(&shelllib);
                    let str: String = "\nimport java.lang.Process;\nimport java.io.InputStream;\nimport java.util.Scanner;\nimport java.text.SimpleDateFormat;\nimport java.util.Date;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.IO.FILEIO" {
                    let fileiolib: String = read_to_string("STD/ALNOOR_J/IO/fileio.ryx")
                        .expect("STD.IO.FILEIO Library Missing");
                    aditlibs.push_str(&fileiolib);
                    let str: String = "\nimport java.io.File;\nimport java.io.FileReader;\nimport java.io.BufferedReader;\nimport java.io.IOException;\nimport java.nio.file.Files;\nimport java.nio.file.Path;\nimport java.nio.file.StandardOpenOption;\nimport java.nio.file.Paths;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.MATH.HASH" {
                    let hashlib: String = read_to_string("STD/ALNOOR_J/MATH/hash.ryx")
                        .expect("STD.MATH.HASH Library Missing");
                    aditlibs.push_str(&hashlib);
                } else if import == "STD.IO.FORMAT" {
                    let str: String = "\nimport java.text.MessageFormat;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.FUNC.RELAY" {
                    let relaylib: String = read_to_string("STD/ALNOOR_J/FUNC/relay.ryx")
                        .expect("STD.FUNC.RELAY Library Missing");
                    aditlibs.push_str(&relaylib);
                } else if import == "STD.FUNC.LAMBDA" {
                    let lmdlib: String = read_to_string("STD/ALNOOR_J/FUNC/lambda.ryx")
                        .expect("STD.FUNC.LAMBDA Library Missing");
                    libs.push_str(&lmdlib);
                } else if import == "STD.FUNC.MACRO" {
                    let mplib: String = read_to_string("STD/ALNOOR_J/FUNC/macro.ryx")
                        .expect("STD.FUNC.MACRO Library Missing");
                    libs.push_str(&mplib);
                    let str: String =
                        "\nimport java.util.ArrayList;\nimport java.util.HashMap;".to_string();
                    imported.push_str(&str);
                    usescan = true;
                } else if import == "STD.FUNC.PIPE" {
                    let pplib: String = read_to_string("STD/ALNOOR_J/FUNC/pipe.ryx")
                        .expect("STD.FUNC.PIPE Library Missing");
                    libs.push_str(&pplib);
                    let str: String = "\nimport java.util.function.BiFunction;\nimport java.util.function.Function;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.FUNC.COMPREHENSION" {
                    let compresslib: String = read_to_string("STD/ALNOOR_J/FUNC/compress.ryx")
                        .expect("STD.FUNC.COMPREHENSION Library Missing");
                    let compresshlib: String = read_to_string("STD/ALNOOR_J/FUNC/compressh.ryx")
                        .expect("STD.FUNC.COMPREHENSION Library Missing");
                    aditlibs.push_str(&compresslib);
                    libs.push_str(&compresshlib);
                } else if import == "STD.STAT.DESK" {
                    let desklib: String = read_to_string("STD/ALNOOR_J/STAT/desk.ryx")
                        .expect("STD.STAT.DESK Library Missing");
                    libs.push_str(&desklib);
                    let str: String = "\nimport java.util.ArrayList;\nimport java.util.Arrays;\nimport java.util.Collections;\nimport java.util.List;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.STAT.LGRAPH" {
                    let lglib: String = read_to_string("STD/ALNOOR_J/STAT/lgraph.ryx")
                        .expect("STD.STAT.LGRAPH Library Missing");
                    libs.push_str(&lglib);
                    let str: String =
                        "\nimport java.awt.BasicStroke;\nimport java.awt.Color;\nimport java.awt.Graphics;\nimport java.awt.Graphics2D;\nimport java.awt.RenderingHints;\nimport java.util.Arrays;\nimport javax.swing.JFrame;\nimport javax.swing.JPanel;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.STAT.BGRAPH" {
                    let bglib: String = read_to_string("STD/ALNOOR_J/STAT/bgraph.ryx")
                        .expect("STD.STAT.BGRAPH Library Missing");
                    libs.push_str(&bglib);
                    let str: String = "\nimport java.awt.Color;\nimport java.awt.Dimension;\nimport java.awt.Font;\nimport java.awt.FontMetrics;\nimport java.awt.Graphics;\nimport javax.swing.JFrame;\nimport javax.swing.JPanel;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.STAT.SGRAPH" {
                    let sglib: String = read_to_string("STD/ALNOOR_J/STAT/sgraph.ryx")
                        .expect("STD.STAT.SGRAPH Library Missing");
                    libs.push_str(&sglib);
                    let str: String = "\nimport java.awt.BasicStroke;\nimport java.awt.Color;\nimport java.awt.Dimension;\nimport java.awt.FontMetrics;\nimport java.awt.Graphics;\nimport java.awt.Graphics2D;\nimport java.awt.Point;\nimport java.awt.RenderingHints;\nimport java.awt.Stroke;\nimport java.util.ArrayList;\nimport java.util.List;\nimport javax.swing.JFrame;\nimport javax.swing.JPanel;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.MATH.EQU" {
                    let mathlib: String = read_to_string("STD/ALNOOR_J/MATH/equations.ryx")
                        .expect("STD.MATH.EQU Library Missing");
                    libs.push_str(&mathlib);
                    let str: String =
                        "\nimport java.util.ArrayList;\nimport java.util.HashMap;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.MATH.SQRT" {
                    let sqrtlib: String = read_to_string("STD/ALNOOR_J/MATH/sqrts.ryx")
                        .expect("STD.MATH.SQRT Library Missing");
                    aditlibs.push_str(&sqrtlib);
                } else if import == "STD.MATH.COLOUR" {
                    let colourlib: String = read_to_string("STD/ALNOOR_J/MATH/colour.ryx")
                        .expect("STD.MATH.COLOUR Library Missing");
                    aditlibs.push_str(&colourlib);
                } else if import == "STD.MATH.RANDOM" {
                    let randomlib: String = read_to_string("STD/ALNOOR_J/MATH/random.ryx")
                        .expect("STD.MATH.RANDOM Library Missing");
                    aditlibs.push_str(&randomlib);
                    let str: String = "\nimport java.util.Random;".to_string();
                    imported.push_str(&str);
                } else if import == "STD.MATH.CLAMP" {
                    let clamplib: String = read_to_string("STD/ALNOOR_J/MATH/clamp.ryx")
                        .expect("STD.MATH.CLAMP Library Missing");
                    aditlibs.push_str(&clamplib);
                } else if import == "STD.MATH.MATRIX" {
                    let matrixlib: String = read_to_string("STD/ALNOOR_J/MATH/matrix.ryx")
                        .expect("STD.MATH.MATRIX Library Missing");
                    aditlibs.push_str(&matrixlib);
                } else {
                    let splitted_import: Vec<&str> = import.split(".").collect();
                    if splitted_import[0] == "java" || splitted_import[0] == "javax" {
                        let str: String = format!("import {};", import);
                        imported.push_str(&str);
                    } else if splitted_import[1] == "ryx" || splitted_import[1] == "java" {
                        let error: String = format!("Unable to read import: {}", import);
                        let importfile: String = read_to_string(import).expect(&error);
                        aditlibs.push_str(&importfile);
                    } else if splitted_import[1] == "lsmx" {
                        let error: String = format!("Unable to read import: {}", import);
                        let importfile: String = read_to_string(import).expect(&error);
                        ext.push_str(&importfile);
                    } else {
                        panic!(
                            "Imported file must end in \".ryx\", \".lsmx\", \".java\", be a Java import or be part of the ALNOOR library."
                        );
                    }
                }
            }
        }

        if usescan {
            let str: String = "\nimport java.util.Scanner;".to_string();
            imported.push_str(&str);
        }
    } else {
        println!("WARNING: Nolib enabled");
    }

    // }

    // PREPROCESSING {

    let mut use_pure = false;
    let mut use_c = false;

    if file_contents.contains("#using pure") {
        use_pure = true;
        println!("WARNING: Using pure");
    } else if file_contents.contains("#using c") {
        use_c = true;
    }

    let self_dund = format!("var _self = new {}();", namef);

    if file_contents.contains("#using arabic") {
        file_contents = file_contents
            .replace("بينما", "while")
            .replace("لو", "if")
            .replace("اخر", "else")
            .replace("قال", "println!")
            .replace("#using arabic", "");

        ext = ext
            .replace("بينما", "while")
            .replace("لو", "if")
            .replace("اخر", "else")
            .replace("قال", "println!")
            .replace("#using arabic", "");
    }

    if !use_c {
        file_contents = file_contents
            .replace("fn main()", "public static void main(String[] args)")
            .replace("_fn ", "private static ")
            .replace("fn ", "public static ")
            .replace("ret ", "return ")
            .replace("bool ", "boolean ")
            .replace("_match ", "switch ")
            .replace("elif ", "else if ")
            .replace("const ", "final ")
            .replace("#define ", "private static ")
            .replace("new_self!", &self_dund)
            .replace("exit!", "System.exit(0);")
            .replace("abort!", "System.exit(1);")
            .replace("print!", "System.out.print")
            .replace("println!", "System.out.println")
            .replace("format!", "MessageFormat.format")
            .replace("args!", "args")
            .replace("_construct ", "public ")
            .replace("_class ", "public class ")
            .replace("_catch;", "catch(Exception e) { e.printStackTrace(); }")
            .replace("|>", "pipe")
            .replace("$.", "this.")
            .replace("l>", "->")
            .replace("#using pure", "");

        ext = ext
            .replace("fn main()", "public static void main(String[] args)")
            .replace("_fn ", "private static ")
            .replace("fn ", "public static ")
            .replace("ret ", "return ")
            .replace("bool ", "boolean ")
            .replace("_match ", "switch ")
            .replace("elif ", "else if ")
            .replace("const ", "final ")
            .replace("#define ", "private static ")
            .replace("new_self!", &self_dund)
            .replace("exit!", "System.exit(0);")
            .replace("abort!", "System.exit(1);")
            .replace("print!", "System.out.print")
            .replace("println!", "System.out.println")
            .replace("format!", "MessageFormat.format")
            .replace("args!", "args")
            .replace("_construct ", "public ")
            .replace("_class ", "public class ")
            .replace("_catch;", "catch(Exception e) { e.printStackTrace(); }")
            .replace("|>", "pipe")
            .replace("$.", "this.")
            .replace("l>", "->");
    } else if use_c {}

    // }

    // TRANSPARSING {

    let to_write: String;

    if use_pure {
        to_write = format!(
            "{imports}\n{libraries}\nclass ExFn {{\n{adit}\n{ext}\n}}\n{code}\n\n\n",
            code = file_contents,
            imports = imported,
            adit = aditlibs,
            libraries = libs,
            ext = ext,
        );
    } else if use_c {
        to_write = format!("{libraries}\n{code}", code = file_contents, libraries = libs);
    } else {
        to_write = format!(
            "{imports}\n{libraries}\npublic class {name} {{\n{adit}\n{ext}\n{code}\n\n\n}}",
            name = namef,
            code = file_contents,
            imports = imported,
            adit = aditlibs,
            libraries = libs,
            ext = ext,
        );
    }
    match java_write.write_all(to_write.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display_source, why),
        Ok(_) => println!("Successfully wrote to {}", display_source),
    }

    if use_c {
        fs::rename(format!("{}.java", namef), format!("{}.c", namef))?;
    }
    // }

    // COMPILATION {
    if !use_c {
        let output_class = Command::new("javac") // TO .class
            .arg(javaf)
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

        if output_class.status.success() {
            let output_java = Command::new("jar") // TO .jar
                .args(["cvfm", &jarf, "Manifest.txt", &classf])
                .output()
                .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

            if output_java.status.success() {
                println!("JAR succeeded\n");
            } else {
                let s = String::from_utf8_lossy(&output_java.stderr);

                println!("JAR failed and stderr was:\n{}", s);
            }
        } else {
            let s = String::from_utf8_lossy(&output_class.stderr);

            println!("JAVAC failed and stderr was:\n{}", s);
        }

        let os: &str = env::consts::OS;

        if os == "windows" {
            let cmd: String = format!("@echo off\njava {}\npause>nul\nexit", namef);
            let cmd_path: &Path = Path::new("Command.cmd"); // CREATE Command.cmd
            let display_cmd = cmd_path.display();

            let mut file_command: File = match File::create(&cmd_path) {
                Err(why) => panic!("Could not create {}: {}", display_cmd, why),
                Ok(file_command) => file_command,
            };

            match file_command.write_all(cmd.as_bytes()) {
                Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
                Ok(_) => println!("Successfully wrote to {}", display_cmd),
            }

            let output_run = Command::new("cmd") // START Command.cmd
                .args(["/C", "start Command.cmd"])
                .output()
                .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
            if output_run.status.success() {
                println!("JAVA succeeded");
            } else {
                let s = String::from_utf8_lossy(&output_run.stderr);

                println!("JAVA failed and stderr was:\n{}", s);
            }
            if cleanup_mode.contains("-cmd") {
                remove_file("Command.cmd").expect("Command file delete failed");
            }
        } else {
            let cmd: String = format!(
                "java {}\nread -rsp $'Press enter to continue...\n'\nexit",
                namef
            );
            let cmd_path: &Path = Path::new("Command.sh"); // CREATE Command.sh
            let display_cmd = cmd_path.display();

            let mut file_command: File = match File::create(&cmd_path) {
                Err(why) => panic!("Could not create {}: {}", display_cmd, why),
                Ok(file_command) => file_command,
            };

            match file_command.write_all(cmd.as_bytes()) {
                Err(why) => panic!("Could not write to {}: {}", display_cmd, why),
                Ok(_) => println!("Successfully wrote to {}", display_cmd),
            }

            let output_run = Command::new("./") // START Command.sh
                .arg("Command.sh")
                .output()
                .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));
            if output_run.status.success() {
                let s = String::from_utf8_lossy(&output_run.stdout);

                println!("JAVA succeeded and stdout was:\n{}", s);
            } else {
                let s = String::from_utf8_lossy(&output_run.stderr);

                println!("JAVA failed and stderr was:\n{}", s);
            }
            if cleanup_mode.contains("-cmd") {
                remove_file("Command.sh").expect("Command file delete failed");
            }
        }
    }
    // }

    // COMAND LINE ARGS {}

    let javaf: String = format!("{}.java", namef);
    let classf: String = format!("{}.class", namef);

    let first_cleanup_mode = cleanup_mode.split("-").collect::<Vec<_>>();
    if first_cleanup_mode[1] == "a" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(javaf).expect("Java delete failed");
        if cleanup_mode.contains("-cmd") {
            remove_file(classf).expect("Class delete failed");
        }
    } else if first_cleanup_mode[1] == "m" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if first_cleanup_mode[1] == "c" {
        remove_file(classf).expect("Class delete failed");
    } else if first_cleanup_mode[1] == "j" {
        remove_file(javaf).expect("Java delete failed");
    } else if first_cleanup_mode[1] == "mc" || first_cleanup_mode[1] == "cm" {
        remove_file("Manifest.txt").expect("Manifest delete failed");
        remove_file(classf).expect("Class delete failed");
    } else if first_cleanup_mode[1] == "cj" || first_cleanup_mode[1] == "jc" {
        remove_file(classf).expect("Class delete failed");
        remove_file(javaf).expect("Java delete failed");
    } else if first_cleanup_mode[1] == "jm" || first_cleanup_mode[1] == "mj" {
        remove_file(javaf).expect("Java delete failed");
        remove_file("Manifest.txt").expect("Manifest delete failed");
    } else if first_cleanup_mode[1] == "n" {} else {
        panic!("Invalid cleanup mode!");
    }

    // }

    Ok(())
}
