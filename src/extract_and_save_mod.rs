//! extract_and_save_mod

//use crate::qvs20_schema_mod;
//use crate::qvs20_writer_mod;
//use crate::utils_mod;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use serde_derive::{Deserialize, Serialize};
//use std::fs::File;
//use unwrap::unwrap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CrateData {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VersionData {
    pub crate_id: String,
    pub num: String,
    pub yanked: String,
    pub created_at: String,
    pub version_for_sorting: Option<String>,
}
/*
pub fn extract_and_save() {
    let versions = versions_non_yanked();
    let path = "database/data/crates.csv";
    // crates.csv:
    // created_at,description,documentation,downloads,homepage,id,max_upload_size,name,readme,repository,textsearchable_index_col,updated_at
    let file = unwrap!(File::open(path));
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(file);
    // prepare schema manually
    let schema = qvs20_schema_mod::Schema::new_from_str(
        r#"[crates]
[String][String][String][String][String]
[][][][][]
[name][description][repository][id][last_version]
"#,
    );
    // [Table] [versions[String]1[]1[version]]
    let mut wtr = qvs20_writer_mod::Writer::new(schema);
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let crate_data: CrateData = unwrap!(result);

        wtr.push_string(&crate_data.name);
        wtr.push_string(&crate_data.description);
        wtr.push_string(&crate_data.repository);
        wtr.push_string(&crate_data.id);
        // find first version with this crate_id
        if let Some(vers) = versions.iter().find(|&x| &x.crate_id == &crate_data.id){
            //last_version
            wtr.push_string(&vers.num);
        }
        else{
            //last_version
            wtr.push_string("0.0.0");
        }
    }
    //write vec_crate_data to qvs20 string and then to file
    unwrap!(std::fs::write("crates.qvs20", wtr.bytes_for_file()));
}

pub fn versions_non_yanked()->Vec<VersionData>{
    let mut versions=vec![];
    let path = "database/data/versions.csv";
    // versions.csv:
    // crate_id,crate_size,created_at,downloads,features,id,license,num,published_by,updated_at,yanked
    let file = unwrap!(File::open(path));
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let mut version_data: VersionData = unwrap!(result);
        if &version_data.yanked == "f"{
            version_data.version_for_sorting = Some( utils_mod::version_for_sorting(&version_data.num));
            versions.push(version_data);
        }
    }
    versions.sort_by(|a, b| b.version_for_sorting.partial_cmp(&a.version_for_sorting).unwrap() );
    versions.sort_by(|a, b| a.crate_id.partial_cmp(&b.crate_id).unwrap() );
    //return
    versions
}
*/
