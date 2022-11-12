struct DataHolder<Data> {
    extract: bool,
    data: Option<Data>,
    sub: Option<Box<DataHolder<Data>>>
}

fn walk<Data, F>(d:&mut DataHolder<Data>, mut f: F) where Data : Clone ,  F:FnMut()->Data {
    if d.extract {
        if d.data.is_none() {
            let dd=f();
            d.data=Some(dd);
        }
        if let Some(sub)=&mut d.sub {
            let dd=&mut d.data;
            walk(sub.as_mut(), || dd.clone().unwrap())
        }
    } else {
        let o=&mut d.sub;
        let dd=&mut d.data;
        if let Some(sub)=o {
            if let Some(ddd)=dd {
                walk(sub.as_mut(), || ddd.clone())
            } else {
                walk(sub.as_mut(), || { 
                    let r=f();
                    *dd=Some(r); 
                    dd.clone().unwrap()})
            }
        }
    }
}

fn test() {
    let mut data=DataHolder {extract: false, data: Some(5), sub: None};
    let mut data2=DataHolder {extract: false, data: None, sub: None};
    let mut data3=DataHolder {extract: true, data: None, sub: None};
    data2.sub=Some(Box::new(data3));
    data.sub=Some(Box::new(data2));
    walk(&mut data, ||5);
}