use std::sync::{Mutex, Arc};
use std::collections::{HashMap, BTreeSet};
use std::cmp::Ordering;

pub type Result<T> = std::result::Result<T, i32>;

#[derive(Debug, Default)]
pub struct ClientMessageSender(String);

#[derive(Debug, Default)]
pub struct Subscription {
    pub msg_sender: Arc<Mutex<ClientMessageSender>>,
    pub subject: String,
    pub sid: String,
}

impl Subscription{
    pub fn new(subject:&str,sid:&str,msg_send:Arc<Mutex<ClientMessageSender>>) -> Self {
        Self{
            msg_sender: msg_send,
            subject: subject.to_string(),
            sid: sid.to_string()
        }
    }
}

pub type ArcSub = Arc<Subscription>;
pub type SubVec = Vec<ArcSub>;

#[derive(Debug, Default,Clone)]
pub struct SubWrapper(ArcSub);

impl std::cmp::PartialEq for SubWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl std::cmp::Eq for SubWrapper {}
impl std::cmp::PartialOrd for SubWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SubWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0.as_ref() as *const Subscription as usize;
        let b = other.0.as_ref() as *const Subscription as usize;
        a.cmp(&b)
    }
}


pub trait SubListTrait {
    fn insert(&mut self, sub: ArcSub) -> Result<()>;
    fn remove(&mut self, sub: ArcSub) -> Result<()>;
    fn match_subject(&mut self, subject: &str) -> SubVec;
}
#[derive(Debug, Default)]
pub struct SubList {
    subs: HashMap<String, BTreeSet<SubWrapper>>,
}

impl SubListTrait for SubList {
    fn insert(&mut self, sub: ArcSub) -> Result<()> {
        let set = self.subs.entry(sub.subject.clone()).or_insert(Default::default());
        set.insert(SubWrapper(sub));
        Ok(())
    }

    fn remove(&mut self, sub: ArcSub) -> Result<()> {
        if let Some(set) = self.subs.get_mut(&sub.subject) {
            set.remove(&SubWrapper(sub.clone()));
            if set.is_empty() {
                self.subs.remove(&sub.subject);
            }
        }
        Ok(())
    }

    fn match_subject(&mut self, subject: &str) -> SubVec {
        let mut v = SubVec::new();
        if let Some(set) = self.subs.get(subject) {
            for sub in set {
                v.push(sub.0.clone())
            }
        }
        v
    }
}

#[test]
fn test() {

    let mut list = SubList::default();
    assert_eq!(list.subs.len(),0);
    let sub = Arc::new(
        Subscription::new("test", "c1",
                          Arc::new(Mutex::new(ClientMessageSender::default()))));
    let sub1 = Arc::new(
        Subscription::new("test", "c2",
                          Arc::new(Mutex::new(ClientMessageSender::default()))));

    list.insert(sub.clone());
    assert_eq!(list.subs.len(),1);
    list.remove(sub.clone());
    assert_eq!(list.subs.len(),0);
    list.insert(sub.clone());
    list.insert(sub.clone());
    list.insert(sub1.clone());

    assert_eq!(list.subs.len(),1);

    let result = list.match_subject("test");
    assert_eq!(result.len(),2)

}