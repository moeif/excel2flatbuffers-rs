use std::time::Instant;
// use excel2flatbuffers_rs::UnlockLvConfig_generated;
// use excel2flatbuffers_rs::file_filter;
// use std::path::PathBuf;
use excel2flatbuffers_rs::data::RawTable;
use excel2flatbuffers_rs::file_filter;
// use std::io;
// use std::io::prelude::*;
// use std::fs::File;
use excel2flatbuffers_rs::fbs2code;

extern crate flatbuffers;
use std::fs;
use std::thread;

extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("lang")
                .short("lang")
                .long("lang")
                .value_name("lang")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("excel")
                .short("excel")
                .long("excel")
                .value_name("excel")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fbs")
                .short("fbs")
                .long("fbs")
                .value_name("fbs")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("bytes")
                .long("bytes")
                .value_name("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("code")
                .short("code")
                .long("code")
                .value_name("code")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let lang = matches.value_of("lang").unwrap_or("csharp");
    let fbs_dir = matches.value_of("fbs").unwrap_or("./common/fbs/");
    let bytes_dir = matches.value_of("bytes").unwrap_or("./common/data_output/");
    let excel_dir = matches.value_of("excel").unwrap_or("./common/excels/");
    let lang_code_dir = String::from(
        matches
            .value_of("code")
            .unwrap_or("./common/csharp_output/"),
    );

    // =================================================

    // let excel_dir = "./common/excels/";
    // let fbs_dir = "./common/fbs/";
    // let data_dir = "./common/data_output/";
    // let target_lan_code_dir = "./common/csharp_output/";
    // let target_lan = "csharp";
    // // let rust_code_dir = "./common/rust_output/";
    let file_identifier = Some("WHAT");

    // Create Directories
    fs::create_dir_all(fbs_dir)?;
    fs::create_dir_all(bytes_dir)?;
    fs::create_dir_all(lang_code_dir)?;

    // fbs2code::generate(fbs_dir, rust_code_dir, "rust")?;

    // let compare_data_dir = "F:/DCProjects/Client/Configs/Excel_Bytes/";

    // let path_vec = file_filter::get_all_files(compare_data_dir, "bytes", false);
    // let my_path_vec = file_filter::get_all_files(data_dir, "bytes", false);
    // let path_str_vec: Vec<String> = Vec::new();
    // let mut my_path_str_vec: Vec<String> = Vec::new();

    // for path in my_path_vec.iter(){
    //     let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
    //     my_path_str_vec.push(file_name);
    // }

    // let mut n = 0;
    // for path in path_vec.iter() {
    //     let path_str = String::from(path.file_name().unwrap().to_str().unwrap());
    //     if !my_path_str_vec.contains(&path_str){
    //         println!("Not Contain: {:?}", path_str);
    //         n += 1;
    //     }
    // }
    // println!("Less: {}", n);

    // let s1 = "haha".to_string();
    // let s2 = "haha".to_string();

    let now = Instant::now();
    let excel_path_vec = file_filter::get_all_files(excel_dir, "xlsx", false);
    // let mut table_vec: Vec<RawTable> = Vec::new();
    // for excel_file in excel_path_vec.iter(){
    //     let table = RawTable::new(excel_file.to_str().unwrap());
    //     table_vec.push(table);
    // }
    // println!("ReadExcel: {}", now.elapsed().as_secs_f32());

    // let now = Instant::now();
    // for table in table_vec.iter(){
    //     table.write_to_fbs_file(fbs_dir)?;
    // }
    // println!("Write to fbs: {}", now.elapsed().as_secs_f32());

    // let now = Instant::now();
    // for table in table_vec.iter(){
    //     table.pack_data(data_dir, file_identifier)?;
    // }
    // println!("Pack data: {}", now.elapsed().as_secs_f32());

    // for i in 0..NTHREADS {
    //     // Spin up another thread
    //     children.push(thread::spawn(move || {
    //         println!("this is thread number {}", i);
    //     }));
    // }

    // for child in children {
    //     // Wait for the thread to finish. Returns a result.
    //     let _ = child.join();
    // }

    let mut thread_vec = Vec::new();

    for excel_file in excel_path_vec.iter() {
        let excel_path = String::from(excel_file.to_str().unwrap());
        thread_vec.push(thread::spawn(move || {
            let table = RawTable::new(&excel_path);
            table.write_to_fbs_file(fbs_dir).unwrap();
            table.pack_data(bytes_dir, file_identifier).unwrap();
        }));
    }

    for child in thread_vec {
        let _ = child.join();
    }
    println!("Read Write Pack: {}", now.elapsed().as_secs_f32());

    let now = Instant::now();
    fbs2code::generate(&fbs_dir, &lang_code_dir, lang)?;
    println!("Genrate Target Code: {}", now.elapsed().as_secs_f32());

    // ==================================================================

    // // =============================== Generate Use Code ==================
    //    let mut builder = flatbuffers::FlatBufferBuilder::new();
    //    let name_vec = builder.create_string("name2");
    //    let start = builder.start_table();
    //    builder.push_slot::<i32>(4, 10101, 0);
    //    builder.push_slot::<i32>(6, 3, 0);
    //    builder.push_slot_always::<flatbuffers::WIPOffset<_>>(8, name_vec);
    //    builder.push_slot::<f32>(10, 100.1, 0.0);

    //    let o1 =builder.end_table(start);
    //    let feature1 = o1.value();push_slot_always::<flatbuffers::WIPOffset<_>>(8, name_vec);

    //    let name_vec = builder.create_string("name2");
    //    let start = builder.start_table();
    //    builder.push_slot::<i32>(4, 10102, 0);
    //    builder.push_slot::<i32>(6, 5, 0);
    //    builder.push_slot_always::<flatbuffers::WIPOffset<_>>(8, name_vec);
    //    builder.push_slot::<f32>(10, 100.2, 0.0);
    //    let o2 = builder.end_table(start);
    // //    let feature2 = o2.value();

    //     let data = builder.create_vector(&[o1, o2]);
    //     let start = builder.start_table();
    //     builder.push_slot_always(4, data);
    //     let o = builder.end_table(start);
    //     builder.finish(o, None);
    //     let buf = builder.finished_data();

    //     let mut writer: Box<dyn std::io::Write> = Box::new(BufWriter::new(std::fs::File::create("data.dat")?));
    //     writer.write_all(&buf);

    //    let feature1 = UnlockLvConfig_generated::SingleUnlockLvConfigData::create(
    //        &mut builder,
    //        &UnlockLvConfig_generated::SingleUnlockLvConfigDataArgs{
    //            ID: 10101,
    //            LevelLimit: 3,
    //            Some()
    //        });

    //     let feature2 = UnlockLvConfig_generated::SingleUnlockLvConfigData::create(
    //      &mut builder,
    //      &UnlockLvConfig_generated::SingleUnlockLvConfigDataArgs{
    //          ID: 10102,
    //          LevelLimit: 5,
    //      });

    //     let data = builder.create_vector(&[feature1, feature2]);
    //     let config = UnlockLvConfig_generated::UnlockLvConfig::create(
    //         &mut builder,
    //         &UnlockLvConfig_generated::UnlockLvConfigArgs{
    //             data: Some(data),
    //         });

    //     builder.finish(config, None);
    //     let buf = builder.finished_data();

    // dbg!(buf, buf.len());

    // ====================================================================

    // let path = "./flatbuffers";
    // let output_path = "./src";
    // fbs2code::generate(path, output_path, "rust");

    // ========================== Load Test =====================================
    // let mut f = std::fs::File::open("./common/data_output/UnlockLvConfig.bytes")?;
    // let mut buf = Vec::new();
    // f.read_to_end(&mut buf)?;
    // let config = UnlockLvConfig_generated::get_root_as_unlock_lv_config(&buf[..]);

    // // let config = UnlockLvConfig_generated::get_root_as_unlock_lv_config(buf);
    // let datas = config.data().unwrap();
    // for data in datas {
    //     let id = data.ID();
    //     let level_limited = data.LevelLimit();
    //     // let name = data.Name().unwrap();
    //     // let speed = data.Speed();
    //     dbg!(id, level_limited);
    // }

    // Ok(())
    // ==========================================================================

    // ========================== Lua Test =====================================
    // lua_exec::exec();

    // =========================================================================

    // ========================== Pack data =====================================

    // let spath = "./excels/";
    // let file_paths = file_filter::get_all_files(spath, false);
    // // let excel_path = "./test_excel/Test.xlsx";
    // let output_path = "./output_fbs/";
    // for file_path in file_paths {
    //     let excel_path = file_path.to_str().unwrap();
    //     process_excel(excel_path, output_path)?;
    // }

    // println!("{}", now.elapsed().as_secs());
    // println!("Generate Down!");

    // Ok(())
    // ==========================================================================

    Ok(())
}

// fn process_excel(excel_path: &str, output_path: &str) -> Result<(), std::io::Error> {
//     //println!("process excel: {}", excel_path);
//     let raw_table_vec = excel_parser::read_excel(excel_path);
//     for raw_table in raw_table_vec {
//         let header_vec = raw_table.get_header_vec();
//         let table_name = &raw_table.name;
//         if table_name.starts_with("【") {
//             continue;
//         }
//         excel2fbs::generate(output_path, table_name, header_vec)?;
//     }
//     Ok(())
// }
