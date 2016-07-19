// Play List (M3U8 Format)

use std::default::Default;
use std::str::FromStr;
use std::string::ToString;
use std::collections::BTreeMap;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Tag {
    // Basic Tags
    M3U,
    VERSION(usize),
    // Media Segment Tags
    INF(f64, Option<String>, String), // duration, title(option), uri
    BYTERANGE(usize, Option<usize>),  // length, start
    DISCONTINUITY,
    // KEY(Option<String>, Option<String>, Option<String>),
    KEY,
    MAP,
    PROGRAM_DATE_TIME,
    DATERANGE,
    // Media Playlist Tags
    TARGETDURATION(f64),   // number s is a decimal-integer indicating the target duration in seconds.
    MEDIA_SEQUENCE(usize), // number is a decimal-integer
    DISCONTINUITY_SEQUENCE,
    ENDLIST,
    PLAYLIST_TYPE(String), // <EVENT|VOD>
    I_FRAMES_ONLY,
    // Master Playlist Tags
    MEDIA,
    STREAM_INF(BTreeMap<String, String>, String), // Attrs, URI
    I_FRAME_STREAM_INF,
    SESSION_DATA,
    SESSION_KEY,
    // Media or Master Playlist Tags
    INDEPENDENT_SEGMENTS,
    START,
}

impl Default for Tag {
    fn default() -> Tag {
        Tag::M3U
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        match *self {
            Tag::M3U                  => "#EXTM3U".to_string(),
            Tag::VERSION(ref version) => format!("#EXT-X-VERSION:{}", version),
            Tag::INF(ref duration, ref title, ref uri) => match *title {
                Some(ref title) => {
                    format!("#EXTINF:{},{}\n{}", duration, title, uri)
                },
                None => {
                    format!("#EXTINF:{},\n{}", duration, uri)
                }
            },
            Tag::BYTERANGE(ref length, ref start) => {
                if start.is_none(){
                    format!("#EXT-X-BYTERANGE:{}", length)
                } else {
                    format!("#EXT-X-BYTERANGE:{}@{}", length, start.unwrap())
                }
            },
            Tag::DISCONTINUITY  => "#EXT-X-DISCONTINUITY".to_string(),
            Tag::KEY            => "#EXT-X-KEY".to_string(),
            Tag::MAP            => "#EXT-X-MAP".to_string(),
            Tag::PROGRAM_DATE_TIME  => "#EXT-X-PROGRAM-DATE-TIME".to_string(),
            Tag::DATERANGE          => "#EXT-X-DATERANGE".to_string(),
            Tag::TARGETDURATION(ref duration) => format!("#EXT-X-TARGETDURATION:{}", duration),
            Tag::MEDIA_SEQUENCE(ref seq)      => format!("#EXT-X-MEDIA-SEQUENCE:{}", seq),
            Tag::DISCONTINUITY_SEQUENCE       => "#EXT-X-DISCONTINUITY-SEQUENCE".to_string(),
            Tag::ENDLIST                      => "#EXT-X-ENDLIST".to_string(),
            Tag::PLAYLIST_TYPE(ref t)         => format!("#EXT-X-PLAYLIST-TYPE:{}", t),
            Tag::I_FRAMES_ONLY                => "#EXT-X-I-FRAMES-ONLY".to_string(),
            Tag::MEDIA                          => "#EXT-X-MEDIA".to_string(),
            Tag::STREAM_INF(ref attrs, ref uri) => {
                let mut _attrs: Vec<String> = Vec::new();
                for (key, value) in attrs {
                    _attrs.push(format!("{}={}", key, value))
                }
                format!("#EXT-X-STREAM-INF:{}\n{}", _attrs.join(","), uri)
            },
            Tag::I_FRAME_STREAM_INF     => "#EXT-X-I-FRAME-STREAM-INF".to_string(),
            Tag::SESSION_DATA           => "#EXT-X-SESSION-DATA".to_string(),
            Tag::SESSION_KEY            => "#EXT-X-SESSION-KEY".to_string(),
            Tag::INDEPENDENT_SEGMENTS   => "#EXT-X-INDEPENDENT-SEGMENTS".to_string(),
            Tag::START                  => "#EXT-X-START".to_string(),
        }
    }
}



impl FromStr for Tag {
    type Err = ();
    fn from_str(s: &str) -> Result<Tag, ()> {
        let s = s.trim();
        if s.starts_with("EXT") == false {
            return Err(());
        }

        if s.starts_with("EXTM3U") {
            Ok(Tag::M3U)
        } else if s.starts_with("EXT-X-VERSION") {
            let kv: Vec<&str> = s.split(":").collect();
            if kv.len() != 2 {
                Err(())
            } else {
                match usize::from_str(kv[1]){
                    Ok(version) => Ok(Tag::VERSION(version)),
                    Err(_)      => Err(())
                }
            }
        } else if s.starts_with("EXTINF") {
            let kv: Vec<&str> = s.split(|c|{c==':'||c==','||c=='\n'}).collect();
            if kv.len() >= 3 {
                let duration: f64 = match f64::from_str(kv[1]){
                    Ok(duration) => duration,
                    Err(_) => return Err(())
                };
                if kv.len() == 3 {
                    let title: Option<String> = None;
                    let uri: String = kv[2].to_string();
                    Ok(Tag::INF(duration, title, uri) )
                } else if kv.len() == 4 {
                    let title: Option<String> = Some(kv[2].to_string());
                    let uri: String = kv[3].to_string();
                    Ok(Tag::INF(duration, title, uri) )
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else if s.starts_with("EXT-X-BYTERANGE") {
            // #EXT-X-BYTERANGE:69864
            // #EXT-X-BYTERANGE:82112@752321
            // #EXT-X-BYTERANGE:75232@0
            let kv: Vec<&str> = s.split(|c|{c==':'||c=='@'}).collect();
            if kv.len() >= 2 {
                let bytes_length: usize = match usize::from_str(kv[1]){
                    Ok(bytes_length) => bytes_length,
                    Err(_) => return Err(())
                };
                if kv.len() == 2 {
                    let start: Option<usize> = None;
                    Ok(Tag::BYTERANGE(bytes_length, start) )
                } else if kv.len() == 3 {
                    let start: usize = match usize::from_str(kv[2]){
                        Ok(start)    => start,
                        Err(_)       => return Err(())
                    };
                    Ok(Tag::BYTERANGE(bytes_length, Some(start)) )
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else if s.starts_with("EXT-X-DISCONTINUITY") {
            Ok(Tag::DISCONTINUITY)
        } else if s.starts_with("EXT-X-KEY") {
            Ok(Tag::KEY)
        } else if s.starts_with("EXT-X-MAP") { 
            Ok(Tag::MAP)
        } else if s.starts_with("EXT-X-PROGRAM-DATE-TIME"){ 
            Ok(Tag::PROGRAM_DATE_TIME)
        } else if s.starts_with("EXT-X-DATERANGE") {
            Ok(Tag::DATERANGE)
        } else if s.starts_with("EXT-X-TARGETDURATION") {
            // EXT-X-TARGETDURATION:12
            let kv: Vec<&str> = s.split(':').collect();
            if kv.len() == 2 {
                let duration: f64 = match f64::from_str(kv[1]){
                    Ok(duration) => duration,
                    Err(_) => return Err(())
                };
                Ok(Tag::TARGETDURATION(duration))
            } else {
                Err(())
            }
        } else if s.starts_with("EXT-X-MEDIA-SEQUENCE") {
            // "EXT-X-MEDIA-SEQUENCE:1"
            let kv: Vec<&str> = s.split(':').collect();
            if kv.len() == 2 {
                let seq: usize = match usize::from_str(kv[1]){
                    Ok(seq) => seq,
                    Err(_) => return Err(())
                };
                Ok(Tag::MEDIA_SEQUENCE(seq))
            } else {
                Err(())
            }
        } else if s.starts_with("EXT-X-DISCONTINUITY-SEQUENCE") {
            Ok(Tag::DISCONTINUITY_SEQUENCE)
        } else if s.starts_with("EXT-X-ENDLIST") {
            Ok(Tag::ENDLIST)
        } else if s.starts_with("EXT-X-PLAYLIST-TYPE") {
            let kv: Vec<&str> = s.split(':').collect();
            if kv.len() == 2 {
                let play_list_type: String = match kv[1] {
                    "EVENT" => "EVENT".to_string(),
                    "VOD"   => "VOD".to_string(),
                    _       => return Err(())
                };
                Ok(Tag::PLAYLIST_TYPE(play_list_type))
            } else {
                Err(())
            }
        } else if s.starts_with("EXT-X-I-FRAMES-ONLY") {
            Ok(Tag::I_FRAMES_ONLY)
        } else if s.starts_with("EXT-X-MEDIA"){
            Ok(Tag::MEDIA)
        } else if s.starts_with("EXT-X-STREAM-INF") {
            // #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=300000\nchunklist-b300000.m3u8
            // attrs: 
            //    Required:
            //        BANDWIDTH, CODECS, 
            //    OPTIONAL:
            //        RESOLUTION(recommended), FRAME-RATE(recommended), 
            //        AVERAGE-BANDWIDTH, AUDIO, VIDEO, SUBTITLES, CLOSED-CAPTIONS,
            let mut kv: Vec<&str> = s.split(|c|{c==':'||c==','||c=='\n'}).collect();
            let uri_index = kv.len()-1;
            let uri = kv[uri_index].to_string();
            kv.remove(uri_index);
            kv.remove(0);

            let mut attrs: BTreeMap<String, String> = BTreeMap::new();
            for _attr in kv.iter(){
                let attr: Vec<&str> = _attr.split('=').collect();
                if attr.len() != 2 {
                    return Err(());
                }
                let key: &str   = attr[0];
                let value: &str = attr[1];
                attrs.insert(key.to_string(), value.to_string());
            }

            Ok(Tag::STREAM_INF(attrs, uri))
        } else if s.starts_with("EXT-X-I-FRAME-STREAM-INF") {
            Ok(Tag::I_FRAME_STREAM_INF)
        } else if s.starts_with("EXT-X-SESSION-DATA") {
            Ok(Tag::SESSION_DATA)
        } else if s.starts_with("EXT-X-SESSION-KEY") {
            Ok(Tag::SESSION_KEY)
        } else if s.starts_with("EXT-X-INDEPENDENT-SEGMENTS") {
            Ok(Tag::INDEPENDENT_SEGMENTS)
        } else if s.starts_with("EXT-X-START") {
            Ok(Tag::START)
        } else {
            Err(())
        }
    }
}

impl Tag {

}