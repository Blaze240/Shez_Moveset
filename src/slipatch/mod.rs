use arcropolis_api::arc_callback;
use arcropolis_api::hash40;
use smash_sli;
const MAX_FILE_SIZE: usize = 0x7A120;


#[arc_callback]
fn sli_patch(hash: u64, data: &mut [u8]) -> Option<usize> {
    if hash == hash40("sound/param/soundlabelinfo.sli").as_u64() {
        let mut buff = std::io::Cursor::new(data);
        match smash_sli::SliFile::read(&mut buff)
        {
            Ok(f) => {
                let mut sli = f;
                sli.entries.push(smash_sli::SliEntry {
                    tone_name: smash_sli::Hash40(hash40("vc_cloud_win04").as_u64()),
                    nus3bank_id: 4587,
                    tone_id: 49,
                });
                match sli.write(&mut buff)
                {
                    Ok(f) => { Some(buff.position() as usize) }
                    Err(e) => None,
                }

            }
            Err(e) => None,
        }
    }
    else {
        None
    }
}


pub fn install() {
    sli_patch::install("sound/param/soundlabelinfo.sli", MAX_FILE_SIZE);
}