fn pct(value) {
    (100.0 * value).to_int() + "%"
}

fn analysis(analysis, progress) {
    let PITCHES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",];
    let texts = ["Unable to find section."];
    for section in analysis["sections"] {
        if section["start"] >= progress {
            let section_key = section["key"];
            let pitch = PITCHES[section_key];
            let tempo = "Tempo: ";
            let section_tempo = section["tempo"];
            let confidence = section["confidence"];
            let confidence_pct = pct(confidence);
            tempo += section_tempo + " (confidence ";
            tempo += confidence_pct + ")";

            let key_confidence = section["key_confidence"];
            let key_confidence_pct = pct(key_confidence);
            let key = "Key: " + pitch + " (confidence ";
            key += key_confidence_pct + ")";

            let time_signature = section["time_signature"];
            let time_signature_confidence = section["time_signature_confidence"];
            let time_signature_confidence_pct = pct(time_signature_confidence);
            let signature = "Time Signature: " + time_signature  + "/4 (confidence " + time_signature_confidence_pct + ")";
            let temp_texts = [tempo, key, signature];
            texts = temp_texts;
            break;
        }
    }
    texts
}

fn draw(analysis, progress) {
    let PITCHES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",];
    let beats = analysis["beats"];
    let segments = analysis["segments"];
    let beat = beats.filter(|beat| beat["start"] >= progress).shift();
    let segment = segments.filter(|segment| segment["start"] >= progress).shift();

    if type_of(beat) == "map" && type_of(segment) == "map" {
        let beat_offset = beat["start"] - progress;
        beat_offset = beat_offset.to_int();
        let labels = segment["pitches"].map(|pitch, index| {
            PITCHES[index]
        });
        let counts = segment["pitches"].map(|pitch| {
            let hz = (pitch * 1000.0).to_int();
            hz + beat_offset
        });
        #{"error":false, "labels": labels, "counts": counts}
    } else {
        #{"error":true, "labels": [], "counts": []}
    }
}
