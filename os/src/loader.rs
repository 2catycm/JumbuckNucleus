//! Loading user applications into memory

/// Get the total number of applications.
use alloc::vec::Vec;
use core::cmp::min;
use lazy_static::*;
///get app number
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}
/// get applications data
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

lazy_static! {
    ///All of app's name
    static ref APP_TUPLES : (Vec<&'static str>, Vec<usize>) = {
        let num_app = get_num_app();
        extern "C" {
            fn _app_names();
        }
        let mut start = _app_names as usize as *const u8;
        let mut names = Vec::new();
        let mut sizes = Vec::new();
        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                while end.read_volatile() != b'\0' {
                    end = end.add(1);
                }
                let size = end as usize- start as usize;
                let slice = core::slice::from_raw_parts(start, size);
                sizes.push(size);
                let str = core::str::from_utf8(slice).unwrap();
                names.push(str);
                start = end.add(1);
            }
        }
        (names, sizes)
    };
}

#[allow(unused)]
///get app data from name
pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let num_app = get_num_app();
    (0..num_app)
        .find(|&i| APP_TUPLES.0[i] == name)
        .map(get_app_data)
}
///list all apps
pub fn sys_ls() ->usize {
    color_println!(34, "/**** App List ****");
    let mut total = 0;
    let len = min(APP_TUPLES.0.len(), APP_TUPLES.1.len());
    for i in 0..len{
        println!("{:>5} Bytes\t{}", APP_TUPLES.1[i], APP_TUPLES.0[i]);
        total+=APP_TUPLES.1[i];
    }
    println!("total {} Bytes", total);
    color_println!(34,"**************/");
    len
}
