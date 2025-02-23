use std::time::{SystemTime, UNIX_EPOCH};

use linkify::{LinkFinder, LinkKind};
use std::fs;

use crate::{dbio, pasta::PastaFile, Pasta};

pub fn remove_expired(pastas: &mut Vec<Pasta>) {
    // get current time - this will be needed to check which pastas have expired
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    pastas.retain(|p| {
        // expiration is `never` or not reached
        if p.expiration == 0 || p.expiration > timenow {
            // keep
            true
        } else {
            // remove the file itself
            if let Some(PastaFile { name, .. }) = &p.file {
                if fs::remove_file(format!("./pasta_data/{}/{}", p.id_as_animals(), name)).is_err()
                {
                    log::error!("Failed to delete file {}!", name)
                }

                // and remove the containing directory
                if fs::remove_dir(format!("./pasta_data/{}/", p.id_as_animals())).is_err() {
                    log::error!("Failed to delete directory {}!", name)
                }
            }
            false
        }
    });

    dbio::save_to_file(pastas);
}

pub fn is_valid_url(url: &str) -> bool {
    let finder = LinkFinder::new();
    let spans: Vec<_> = finder.spans(url).collect();
    spans[0].as_str() == url && Some(&LinkKind::Url) == spans[0].kind()
}
