use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    stage_commonlib_test_address_libraries()
        .expect("failed to stage CommonLib test address libraries");

    #[cfg(not(feature = "prebuilt"))]
    xmake_build();
    #[cfg(feature = "prebuilt")]
    fetch_libs();

    println!("cargo:rustc-link-lib=static=commonlib_bridge");
    println!("cargo:rustc-link-lib=static=commonlibsse-ng");

    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=bcrypt");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=dbghelp");

    println!("cargo:rustc-link-lib=d3d11");
    println!("cargo:rustc-link-lib=dxgi");
    println!("cargo:rustc-link-lib=d3dcompiler");

    println!("cargo:rerun-if-changed=cpp/src");
    println!("cargo:rerun-if-changed=cpp/include");
    println!("cargo:rerun-if-changed=cpp/xmake.lua");
}

fn stage_commonlib_test_address_libraries() -> io::Result<()> {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set"));
    let source_dir = manifest_dir
        .join("..")
        .join("CommonLibVR")
        .join("tests")
        .join("REL");
    if !source_dir.exists() {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is always set"));
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR should always be nested under target/<profile>/build/<pkg>/out");
    let destination_dir = profile_dir
        .join("deps")
        .join(Path::new("Data").join("SKSE").join("Plugins"));
    fs::create_dir_all(&destination_dir)?;

    for entry in fs::read_dir(&source_dir)? {
        let entry = entry?;
        let source_path = entry.path();
        let Some(file_name) = source_path.file_name() else {
            continue;
        };

        let matches_address_library = matches!(
            source_path.extension().and_then(|ext| ext.to_str()),
            Some("bin" | "csv")
        ) && file_name.to_string_lossy().starts_with("version");
        if !matches_address_library {
            continue;
        }

        fs::copy(&source_path, destination_dir.join(file_name))?;
        println!("cargo:rerun-if-changed={}", source_path.display());
    }
    Ok(())
}

/// Download C++ libraries
#[cfg(feature = "prebuilt")]
fn fetch_libs() {
    use std::io::Cursor;

    let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lib_root = crate_root.join("cpp").join("build");
    if std::fs::exists(lib_root.join("commonlib_bridge.lib")).unwrap_or(false) {
        println!("cargo:rustc-link-search=native={}", lib_root.display());
        return;
    }
    let out_dir = lib_root.as_path();
    std::fs::create_dir_all(out_dir).unwrap();

    let url = format!(
        "https://github.com/SARDONYX-forks/CommonLib-SkyrimLib/releases/download/v2.2.3/commonlib_bridge.zip",
    );

    // Download zip(Wait up to 30 minutes to download 160 MB considering the slow network.)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 30))
        .build()
        .unwrap();
    let response = client
        .get(&url)
        .send()
        .unwrap_or_else(|e| panic!("Failed to download ZIP. url: {url}, err: {e}"));
    let bytes = response.bytes().expect("Failed to read response bytes");

    let mut archive =
        zip::read::ZipArchive::new(Cursor::new(bytes)).unwrap_or_else(|err| panic!("{err}"));
    archive
        .extract(out_dir)
        .unwrap_or_else(|err| panic!("{err}"));

    println!("cargo:rustc-link-search=native={}", lib_root.display());
}

#[cfg(not(feature = "prebuilt"))]
fn xmake_build() {
    let mut config = xmake::Config::new("cpp");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_release_like = profile.starts_with("release") || profile == "bench";
    let build_mode = if is_release_like { "release" } else { "debug" };
    config.mode(build_mode);

    let is_se = env::var("CARGO_FEATURE_SE").is_ok();
    let is_ae = env::var("CARGO_FEATURE_AE").is_ok();
    let is_vr = env::var("CARGO_FEATURE_VR").is_ok();

    config.option("skyrim_se", if is_se { "y" } else { "n" });
    config.option("skyrim_ae", if is_ae { "y" } else { "n" });
    config.option("skyrim_vr", if is_vr { "y" } else { "n" });
    config.option("skse_xbyak", "y");

    config.build();

    let build_info = config.build_info();
    let dst = build_info
        .linkdirs()
        .iter()
        .find(|dir| {
            let dir = dir.as_path();
            dir.join("commonlib_bridge.lib").exists()
                && dir.to_string_lossy().contains(build_mode)
        })
        .unwrap_or_else(|| {
            let available = build_info
                .linkdirs()
                .iter()
                .map(|dir| dir.display().to_string())
                .collect::<Vec<_>>()
                .join("; ");
            panic!(
                "xmake returned no artifact link directory for profile={profile}, mode={build_mode}; available linkdirs: {available}"
            )
        });
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=spdlog");
}
