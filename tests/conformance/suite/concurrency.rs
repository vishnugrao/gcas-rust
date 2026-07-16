use std::sync::Arc;
use std::thread;

use super::support::ramp;
use gcas_rust::contract::ContentStore;
use gcas_rust::id::ContentId;

pub fn concurrent_puts_agree_on_address<S: ContentStore + 'static>(s: Arc<S>) {
    let data = ramp(8192);
    let expected = ContentId::of(&data);

    let handles: Vec<_> = (0..16)
        .map(|_| {
            let s = Arc::clone(&s);
            let data = data.clone();
            thread::spawn(move || s.put(&data).unwrap())
        })
        .collect();

    for handle in handles {
        assert_eq!(handle.join().unwrap(), expected);
    }
    assert!(s.has(&expected).unwrap());
}
