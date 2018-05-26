


pub mod renderable;
pub mod gameobject;
pub mod component;
pub mod player_controller;
pub mod renderable_text;



use std::sync::Mutex;
use ggez::event::*;
use ggez::Context;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::sync::MutexGuard;
use lazy_static::lazy::Lazy;
use ecsystem::gameobject::GameObject;

pub struct Input {
    pub keycode_down: Option<Keycode>,
    pub keymod_down: Option<Mod>,
    pub keycode_up: Option<Keycode>,
    pub keymod_up: Option<Mod>,
}

lazy_static! {
    pub static ref GAME_OBJECTS: Mutex<HashMap<String, Box<GameObject>>> = Mutex::new(HashMap::new());
    pub static ref TAGS: Mutex<HashSet<String>> = Mutex::new(HashSet::new()); 
}

pub struct ECSystem {

    //pub game_objects: HashMap<String, Box<GameObject>>,
    pub input: Input,
}


impl ECSystem {
    pub fn new() -> Self {
        println!("new ecsystem");
        let ecs = ECSystem{
            //game_objects: HashMap::new(),
            input: Input {keycode_down: None, keymod_down: None, keycode_up: None, keymod_up: None},

        };
        ecs
    }

    pub fn add_game_object(&mut self, go: Box<GameObject>) {
        let tag = go.tag.clone();
        TAGS.lock().unwrap().insert(tag.clone());
        
        GAME_OBJECTS.lock().unwrap().insert(tag.clone(), go);
    }

    //pub fn get_game_object<'a>(&'a mut self, tag: String) -> &'a mut GameObject {
        //GAME_OBJECTS.lock().unwrap().get_mut(&tag).unwrap()
    //}


    pub fn update(&mut self) {
       let tags = TAGS.lock().unwrap();
        for tag in tags.iter() {
            //GAME_OBJECTS.lock().unwrap()[tag].update(self);
            GAME_OBJECTS.lock().unwrap().get_mut(&tag.clone()).unwrap().update(self);
        }
    }    
    
    pub fn render(&mut self, ctx: &mut Context) {
        let tags = TAGS.lock().unwrap();
        for tag in tags.iter() {
            GAME_OBJECTS.lock().unwrap()[tag].render(ctx);
        }
    }

    //pub fn game_objects() -> &Result<MutexGuard<HashMap<String, Box<GameObject>>>> {
        //&GAME_OBJECTS.lock()
        
    //}

    //pub fn find(tag: String) -> EntryPlace<'static, String, Box<GameObject>> {
    //pub fn find(tag: String) {
        
        //GAME_OBJECTS.lock().unwrap().entry(tag).make_place()
        
        
    //}


}