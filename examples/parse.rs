
extern crate hls;

use std::str::FromStr;
use hls::{Playlist, Tag};

#[allow(non_upper_case_globals, dead_code)]
static Master_Playlist: &'static str = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:12
#EXT-X-MEDIA-SEQUENCE:1
#EXTINF:12.0,Title 1
media-b2000000_1.ts?wowzasessionid=2029972411
#EXTINF:12.0,Title 2
media-b2000000_2.ts?wowzasessionid=2029972411
#EXTINF:12.0,
media-b2000000_3.ts?wowzasessionid=2029972411
#EXTINF:12.0,
media-b2000000_4.ts?wowzasessionid=2029972411
#EXTINF:12.0,
media-b2000000_5.ts?wowzasessionid=2029972411
#EXTINF:12.0,
media-b2000000_6.ts?wowzasessionid=2029972411
#EXTINF:12.0,
media-b2000000_7.ts?wowzasessionid=2029972411
#EXTINF:12.0,
#EXT-X-ENDLIST
"#;


fn main() {
    println!("Master Playlist:\n{}\n\n", Master_Playlist);
    match Playlist::from_str(Master_Playlist){
        Ok(playlist) => {
            for tag in playlist.tags() {
                println!("Tag: {:?}", tag);
            }
        },
        Err(_) => println!("parse error.")
    };
}