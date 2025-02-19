// This comes from the README of `cargo-ndk` as an example of how to link to and copy
// the libc++_shared library to the output folder automatically.

#![allow(clippy::unwrap_used)]
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use xml::writer::{EmitterConfig, XmlEvent};

fn main() {
    // generate jni java bindings
    println!("cargo:rerun-if-changed=src/lib.rs");
    jnigen_build::generate("src/lib.rs", "walletsdk", "tests/src/main/java");
    jnigen_build::generate("src/lib.rs", "walletsdk", "./jar/java");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
        generate_pom_xml();
        generate_settings_xml();
    }
}

macro_rules! write_element {
    ($writer:expr, $name:expr, $value:expr) => {
        $writer
            .write(XmlEvent::start_element($name))
            .expect("Failed to write start element");
        $writer
            .write(XmlEvent::characters($value))
            .expect("Failed to write characters");
        $writer
            .write(XmlEvent::end_element())
            .expect("Failed to write end element");
    };
}

fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        let lib_path = sysroot_libs_path.join("libc++_shared.so");
        let out_folder = Path::new(&output_path).join(env::var("CARGO_NDK_ANDROID_TARGET").unwrap());

        // make sure the output folder exists before copying
        std::fs::create_dir_all(&out_folder).unwrap();

        std::fs::copy(lib_path, out_folder.join("libc++_shared.so")).unwrap();
    } else {
        panic!("Could not copy libc++_shared.so since `CARGO_NDK_OUTPUT_PATH` was not set");
    }
}

fn generate_pom_xml() {
    let version = env!("CARGO_PKG_VERSION");
    let version_info = match env::var("CI_COMMIT_TAG") {
        Ok(_) => version.to_string(),
        Err(_) => format!("{}-SNAPSHOT", version),
    };

    let pom_file_path = Path::new("./jar/pom.xml");
    let file = fs::File::create(pom_file_path).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .create_writer(file);

    writer
        .write(
            XmlEvent::start_element("project")
                .attr("xmlns", "http://maven.apache.org/POM/4.0.0")
                .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
                .attr(
                    "xsi:schemaLocation",
                    "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd",
                ),
        )
        .unwrap();

    write_element!(writer, "modelVersion", "4.0.0");
    write_element!(writer, "groupId", "com.cawaena");
    write_element!(writer, "artifactId", "wallet");
    write_element!(writer, "version", &version_info);
    write_element!(writer, "packaging", "jar");
    write_element!(writer, "name", env!("CARGO_PKG_NAME"));
    write_element!(writer, "description", env!("CARGO_PKG_DESCRIPTION"));

    writer.write(XmlEvent::start_element("licenses")).unwrap();
    writer.write(XmlEvent::start_element("license")).unwrap();
    write_element!(writer, "name", env!("CARGO_PKG_LICENSE"));
    writer.write(XmlEvent::end_element()).unwrap(); // Close license
    writer.write(XmlEvent::start_element("license")).unwrap();
    write_element!(writer, "name", "The Apache Software License, Version 2.0");
    write_element!(writer, "url", "http://www.apache.org/licenses/LICENSE-2.0.txt");
    writer.write(XmlEvent::end_element()).unwrap(); // Close license
    writer.write(XmlEvent::end_element()).unwrap(); // Close licenses

    writer.write(XmlEvent::start_element("developers")).unwrap();
    writer.write(XmlEvent::start_element("developer")).unwrap();
    write_element!(writer, "name", env!("CARGO_PKG_AUTHORS"));
    write_element!(writer, "organization", "ETO GRUPPE TECHNOLOGIES GmbH");
    write_element!(writer, "organizationUrl", "https://www.etogruppe.com/");
    writer.write(XmlEvent::end_element()).unwrap(); // Close developer
    writer.write(XmlEvent::end_element()).unwrap(); // Close developers

    writer.write(XmlEvent::start_element("scm")).unwrap();
    write_element!(writer, "connection", "scm:git:git://github.com/ETOSPHERES-Labs/cawaena-sdk.git");
    write_element!(writer, "developerConnection", "scm:git:ssh://github.com:ETOSPHERES-Labs/cawaena-sdk.git");
    write_element!(writer, "url", ">http://github.com/ETOSPHERES-Labs/cawaena-sdk/tree/master");
    writer.write(XmlEvent::end_element()).unwrap(); // Close scm

    
    writer.write(XmlEvent::start_element("distributionManagement")).unwrap();

    writer.write(XmlEvent::start_element("repository")).unwrap();
    write_element!(writer, "id", "ossrh");
    write_element!(writer, "name", "repo.farmunited.com-releases");
    write_element!(writer, "url", "https://oss.sonatype.org/service/local/staging/deploy/maven2/");
    writer.write(XmlEvent::end_element()).unwrap(); // Close repository

    writer.write(XmlEvent::start_element("snapshotRepository")).unwrap();
    write_element!(writer, "id", "ossrh");
    write_element!(writer, "name", "repo.farmunited.com-snapshots");
    write_element!(writer, "url", "https://oss.sonatype.org/content/repositories/snapshots");
    writer.write(XmlEvent::end_element()).unwrap(); // Close snapshotRepository
    writer.write(XmlEvent::end_element()).unwrap(); // Close distributionManagement

    writer.write(XmlEvent::start_element("properties")).unwrap();
    write_element!(writer, "project.build.sourceEncoding", "UTF-8");
    writer.write(XmlEvent::end_element()).unwrap(); // Close properties

    writer.write(XmlEvent::end_element()).unwrap(); // Close project
}

fn generate_settings_xml() {
    let settings_file_path = Path::new("./jar/settings.xml");
    let file = fs::File::create(settings_file_path).unwrap();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(file);

    writer
        .write(
            XmlEvent::start_element("settings")
                .attr(
                    "xsi:schemaLocation",
                    "http://maven.apache.org/SETTINGS/1.2.0 http://maven.apache.org/xsd/settings-1.2.0.xsd",
                )
                .attr("xmlns", "http://maven.apache.org/SETTINGS/1.2.0")
                .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
        )
        .unwrap();

    writer.write(XmlEvent::start_element("servers")).unwrap();
    writer.write(XmlEvent::start_element("server")).unwrap();
    write_element!(writer, "username", "${env.TOKEN_USERNAME}");
    write_element!(writer, "password", "${env.TOKEN_PASSWORD}");
    write_element!(writer, "id", "ossrh");
    writer.write(XmlEvent::end_element()).unwrap(); // Close server
    writer.write(XmlEvent::end_element()).unwrap(); // Close servers

    writer.write(XmlEvent::start_element("profiles")).unwrap();
    writer.write(XmlEvent::start_element("profile")).unwrap();
    writer.write(XmlEvent::start_element("repositories")).unwrap();
    writer.write(XmlEvent::start_element("repository")).unwrap();
    writer.write(XmlEvent::start_element("snapshots")).unwrap();
    write_element!(writer, "enabled", "false");
    writer.write(XmlEvent::end_element()).unwrap(); // Close snapshots
    write_element!(writer, "id", "ossrh");
    write_element!(writer, "name", "snapshots");
    write_element!(writer, "url", "https://oss.sonatype.org/content/repositories/snapshots");
    writer.write(XmlEvent::end_element()).unwrap(); // Close repository
    writer.write(XmlEvent::end_element()).unwrap(); // Close repositories
    write_element!(writer, "id", "artifactory");
    writer.write(XmlEvent::end_element()).unwrap(); // Close profile
    writer.write(XmlEvent::end_element()).unwrap(); // Close profiles

    writer.write(XmlEvent::start_element("activeProfiles")).unwrap();
    write_element!(writer, "activeProfile", "artifactory");
    writer.write(XmlEvent::end_element()).unwrap(); // Close activeProfiles

    writer.write(XmlEvent::end_element()).unwrap(); // Close settings
}
