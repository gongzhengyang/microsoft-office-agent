use std::collections::HashMap;

const MAX_SPLITS: usize = 8;

/// office 日志采用`\t`切割，最多会被切割为8份
pub fn parse_text(text: &str) -> Vec<HashMap<String, String>> {
    let mut lines = text.lines();
    let first_line = lines.next();
    if first_line.is_none() {
        return vec![];
    }
    let keys = first_line
        .unwrap()
        .splitn(MAX_SPLITS, '\t')
        .collect::<Vec<&str>>();
    let mut results = vec![];
    for line in lines {
        let values = line.splitn(MAX_SPLITS, '\t').collect::<Vec<&str>>();
        if values.len().eq(&MAX_SPLITS) {
            let mut result = HashMap::with_capacity(MAX_SPLITS);
            for (i, v) in values.iter().enumerate() {
                result.insert(keys[i].to_owned(), v.to_string());
            }
            results.push(result);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let text = r#"Timestamp	Process	TID	Area	Category	EventID	Level	Message	Correlation
03/17/2023 07:31:31.046	EXCEL (0x4F8)	0x2194	Microsoft Excel	Telemetry Event	b7vzq	Medium	SendEvent {"EventName":"Office.Text.GDIAssistant.RegisterCloudFontCallback","Flags":30962256044949761,"InternalSequenceNumber":32,"Time":"2023-03-17T07:31:31.046Z","Contract":"Office.System.Activity","Activity.CV":"2vQcCqNtLUeemGt0NEQK/A.1.19","Activity.Duration":11,"Activity.Count":1,"Activity.AggMode":0,"Activity.Success":true}
03/17/2023 07:31:31.437	EXCEL (0x4F8)	0x1C30	Microsoft Excel	Telemetry Event	b7vzq	Medium	SendEvent {"EventName":"Office.Docs.DocumentMru.AggregatedMru.CacheIdentities","Flags":30962256044949761,"InternalSequenceNumber":56,"Time":"2023-03-17T07:31:31.437Z","Contract":"Office.System.Activity","Activity.CV":"2vQcCqNtLUeemGt0NEQK/A.9","Activity.Duration":26,"Activity.Count":1,"Activity.AggMode":0,"Activity.Success":true,"Activity.Result.Code":0,"Activity.Result.Type":"HRESULT","Activity.Result.Tag":42254603,"Data.Reason":0,"Data.MsaSignedInCount":0,"Data.MsaNotSignedInCount":0,"Data.OrgIdSignedInCount":0,"Data.OrgIdNotSignedInCount":0,"Data.ActiveDirectoryCount":0,"Data.SSPICount":0,"Data.OAuth2Count":0,"Data.BadgerCount":0,"Data.TotalCount":0,"Data.HasChanges":false,"Data.StopwatchDuration":12}
"#;
        let results = parse_text(text);
        println!("{:?}", results);
    }
}
