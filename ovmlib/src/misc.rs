

use core::num;
use std::{collections::HashMap};
use futures::future::{BoxFuture, FutureExt};
use tokio::net::tcp;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

// static requires constant value and constant values must be only stack data like &[] and &str otherwise
// we're not allowed to have heap data types like Vec, String, Box, Arc, Mutex in const and static as value
// also in order to mutate an static we should wrap around the arc and mutex to do so but inside lazy
// str is ?Sized and it must be behind pointer cause it's an slice of String, same for [] 
// generally only &[] and &str are allowed to be in const and static value, using other types than these
// requires a global static lazy arc mutex like so:
type SafeMap = Lazy<std::sync::Arc<tokio::sync::Mutex<HashMap<String, String>>>>;
pub static GLOBALMAP: SafeMap = Lazy::new(||{
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            HashMap::new()
        )
    )
});
pub static ONTHEHEAP: &[&str] = CONSTVALUE;
pub const CONSTVALUE: &[&str] = &["wildonion"];
pub const CHARSET: &[u8] = b"0123456789";


// -----------------------------------
// handling a recursive async function
// -----------------------------------
// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
// NOTE - Future trait is an object safe trait thus we have to Box it with dyn keyword to have kinda a pointer to the heap where the object is allocated in runtime
// NOTE - a recursive `async fn` will always return a Future object which must be rewritten to return a boxed `dyn Future` to prevent infinite size allocation in runtime from heppaneing some kinda maximum recursion depth exceeded prevention process
// the return type can also be ... -> impl std::future::Future<Output=usize>
// which implements the future trait for the usize output also BoxFuture<'static, usize>
// is a pinned Box under the hood because in order to return a future as a type
// we have to return its pinned pointer since future objects are traits and 
// traits are not sized at compile time thus we have to put them inside the 
// Box or use &dyn to return them as a type and for the future traits we have
// to pin them into the ram in order to be able to solve them later so we must 
// return the pinned Box (Box in here is a smart pointer points to the future)
// or use impl Trait in function return signature. 
//
// async block needs to be pinned into the ram and since they are traits of 
// the Future their pointer will be either Box<dyn Trait> or &dyn Trait, 
// to pin them into the ram to solve them later.
//
// since async blocks are of type Future trait in roder to return them
// as a type their pointer either Box<dyn Trait> or &dyn Trait must be
// pinned into the ram to let us solve them later because rust doesn't 
// have gc and it'll drop the type after it moved into the new scope or
// another type thus for the future objects we must pin them to ram and 
// tell rust hey we're moving this in other scopes but don't drop it because
// we pinned it to the ram to solve it in other scopes, also it must have
// valid lifetime during the the entire lifetime of the app.
//
// BoxFuture<'fut, ()> is Pin<alloc::boxed::Box<dyn Future<Output=()> + Send + Sync + 'fut>>
pub fn async_gen_random_idx(idx: usize) -> BoxFuture<'static, usize>{ // NOTE - pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>
    async move{
        if idx <= CHARSET.len(){
            idx
        } else{
            gen_random_idx(rand::random::<u8>() as usize)
        }
    }.boxed() // wrap the future in a Box, pinning it
}
pub fn ret_boxed_future() -> std::pin::Pin<Box<dyn std::future::Future<Output=()>>>{ // Pin takes a pointer to the type and since traits are dynamic types thir pointer can be either &dyn ... or Box<dyn...>
    // ret future as a pinned box means pinning the pointer of future trait into the ram so they can't move
    Box::pin(async move{ // pinning the box pointer of async block into the ram to solve it later 
        ()
    })
}

// recursive random index generator
pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARSET.len(){
        idx
    } else{
        gen_random_idx(rand::random::<u8>() as usize)
    }
}

pub struct MerkleNode{}
impl MerkleNode{

    pub fn new() -> Self{
        MerkleNode {  }
    }

    pub fn calculate_root_hash(&mut self, chain: Vec<String>){

    } 
}

#[derive(Debug, Clone)]
pub enum RuntimeCode{
    Err(u8),
    Ok(u8),

}

struct CodePid{
    pub ramdb: HashMap<String, String>
}


/*  ----------------------------------------------------------------------
    implementing a dynamic type handler for structs and enums using traits
    ----------------------------------------------------------------------
*/
trait TypeTrait{
    type Value; // this can be the implementor type

    /* 
        we can use the lifetime of self in struct and trait methods 
        to return pointer since the self is valid as long as the object 
        itself is valid during the execution of the app
    */
    fn get_data(&self) -> Self::Value;
    fn get_ctx_data(&self, ctx: Self::Value) -> Self;
    fn fill_buffer(&mut self) -> &[u8];
}

impl TypeTrait for CodePid{
    type Value = Self; // the CodePid structure

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }
}

impl TypeTrait for MerkleNode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<HashMap<u32, String>>>;

    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                HashMap::new()
            )
        );
        mutexed_data
    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }
}

struct Streamer;
struct Context<T>{data: T}
impl TypeTrait for Streamer{ // kinda polymorphism
    
    type Value = Context<Self>; /* Context data is of type Streamer */

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        ctx.data
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }

}

impl TypeTrait for RuntimeCode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<String>>;
    
    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                String::from("")
            )
        );
        mutexed_data

    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }
}

pub trait NodeReceptor{
    type InnerReceptor;
    fn get_inner_receptor(&self) -> Self::InnerReceptor;
}

pub trait Activation<C>: Send + Sync + 'static + Clone + Default{
    type Acivator;
}

impl<C> Activation<C> for &'static [u8]{
    type Acivator = &'static [u8];
}

#[derive(Default)]
pub struct Synapse<A>{id: A}

#[derive(Default)]
pub struct Neuron<A=u8>{
    pub data: Option<Synapse<A>>,
}

/* 
    this must be implemented for Neuron<Synapse<A>>
    to be able to call get_inner_receptor() method
*/
impl<A: Default> NodeReceptor for Neuron<Synapse<A>>
where Self: Clone + Send + Sync + 'static + Activation<String>, 
<Self as Activation<String>>::Acivator: Default{

    type InnerReceptor = Synapse<A>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        let id: A = Default::default();
        Synapse{
            id,
        }
    }
}

/* 
    this must be implemented for Neuron<String>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<String>{

    type InnerReceptor = Synapse<String>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: String::from(""),
        }
    }
}

/* 
    this must be implemented for Neuron<A>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<u8>{

    type InnerReceptor = Synapse<u8>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: 0,
        }
    }
}

pub fn fire<'valid, N, T: 'valid + NodeReceptor>(cmd: N, cmd_receptor: impl NodeReceptor) 
    -> <N as NodeReceptor>::InnerReceptor // or T::InnerReceptor
    where N: Send + Sync + 'static + Clone + NodeReceptor + ?Sized, 
    T: NodeReceptor, T::InnerReceptor: Send + Clone,
    /* casting generic N to NodeReceptor trait to access the InnerReceptor gat */
    <N as NodeReceptor>::InnerReceptor: Send + Sync + 'static{

    // with pointer we can borrow the type to prevent from moving and 
    // makes the type sizable at compile time by storing the address of 
    // none determined size of it inside the stack like str and []
    // box is sized with the size of its content allocated on the heap
    trait Test{}
    struct Neuronam{}
    let name = Neuronam{};
    impl Test for Neuronam{}
    let trait_name = &name as &dyn Test;
    struct AnotherNeuronam<T: Test, F> where F: FnOnce() -> (){
        pub data: T,
        pub new_data: F
    }
    impl<V: Test, T> AnotherNeuronam<V, T> where T: FnOnce() -> (){
        fn get_data(param: impl FnMut() -> ()) -> impl FnMut() 
            -> std::pin::Pin<Box<dyn std::future::Future<Output=String> + Send + Sync + 'static>>{
            ||{
                Box::pin(async move{
                    String::from("")
                })
            }
        }
        fn get_func() -> fn() -> String{
            fn get_name() -> String{
                String::from("")
            }
            get_name
        }
        }
    let another_name = AnotherNeuronam{data: name, new_data: ||{}};

    let cls = |func: fn() -> String|{
        func()
    };
    fn execute() -> String{
        String::from("wildonion")
    }
    cls(execute);

    let cls = ||{};
    let casted = &cls as &dyn Fn() -> (); // casting the closure to an Fn trait
    let name = (
        |name: String| -> String{
            name
        }
    )(String::from(""));
    
    enum Packet{
        Http{header: String},
        Tcp{size: usize}, // the size of the incoming buffer
        Snowflake{id: String}
    }
    let packet = Packet::Http { header: String::from("") };
    if let Packet::Http { header } = packet{
        println!("packet header bytes => {header:}");
    }

    enum UserName{
        Age,
        Id,
        Snowflake{id: String}
    }
    let enuminstance = (Packet::Tcp{size: 0 as usize}, Packet::Http { header: String::from("http header") });
    let res = match enuminstance{
        (Packet::Tcp { size: tcpsize }, Packet::Http{ header: httpheader }) | 
        (Packet::Http{ header: httpheader }, Packet::Tcp { size: tcpsize }) => {},
        (_, Packet::Snowflake{id: sid}) => if !sid.is_empty(){},
        _ => {}
    };

    /*  
        note that if we want to call get_inner_receptor() method
        on an instance of Neuron, the NodeReceptor trait must be
        implemented for every generic type in Neuron struct separately
        like:
            impl NodeReceptor for Neuron<String>{}
            impl NodeReceptor for Neuron<u8>{}
            impl NodeReceptor for Neuron<Synapse<A>>{}
    */
    let neuron = cmd;
    let neuron_ = Neuron::<String>::default();
    
    cmd_receptor.get_inner_receptor();
    neuron.get_inner_receptor()
    // neuron_.get_inner_receptor()
    
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum ActionType{
    #[default]
    A1
} 
type Method = fn() -> i32;
fn run<'lifetime>(param: impl Fn() -> ActionType, method: &'lifetime Method)
// bounding generic Method to traits and lifetimes
where Method: Send + Sync + 'static{}
fn execute<'f, F>(param: &'f mut F) -> () 
// bounding generic F to closure, lifetimes and other traits
where F: Fn() -> ActionType + Send + Sync + 'static{}

// bounding generic to traits and lifetiems
// async trait fn run in multithread env using #[trait_variant::make(TraitNameSend: Send)]
// bounding trait method only to traits like TraitName::foo(): Send + Sync
// return trait from method using -> impl TraitName
// trait as method param like param: impl TraitName
// trait as struct field like pub data: F (where F: TraitName) or pub data: Box<dyn TraitName> 
// casting generic to trait like &N as &dyn TraitName or N as TraitName
// bounding trait gat to traits like <N as TraitName>::AssetInfo: Send + Sync
// bounding the return type of closure trait to traits like where F: FnOnce() -> R + Send + Sync + 'static
trait Interface: Send + Sync + 'static{}
struct Instance{}
impl Interface for Instance{}
impl Interface for (){}
type BoxedTrait = Box<dyn FnOnce() -> ()>;
struct Test<R, F: Send + Sync + 'static + Clone + Default> 
    where F: FnOnce() -> R + Send + Sync + 'static, 
        R: Send + Sync + 'static{
    pub data: F,
    pub another_data: BoxedTrait
}
fn trait_as_ret_and_param_type(param: &mut impl FnOnce() -> ()) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_and_param_type1(param_instance: &mut impl Interface) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_type(instance_type: Instance) -> impl Interface{ instance_type }
fn trait_as_ret_type_1(instance_type: Instance) -> impl Interface{ () }
fn trait_as_param_type(param: impl FnOnce() -> ()){

    struct Button<T: FnOnce() -> String + Send + Sync>{
        pub onclick: Box<dyn FnOnce() -> T>, // dynamic dispatch using object safe trait by boxing the trait 
    }
    let b = Button{onclick: Box::new(||{ ||{String::from("")} })};
    (b.onclick)();

    trait AuthExt{}
    #[derive(Clone)]
    struct Auth{}
    impl AuthExt for Auth{}
    impl Auth{
        fn get_trait(&self) -> &(dyn AuthExt){
            let t = self as &dyn AuthExt; // use casting
            t 
            // &Self{}
        }
        fn get_trait1(&self) -> impl AuthExt{
            Self{}
        }
        fn get_trait2(&self) -> Box<dyn AuthExt>{
            let t = Box::new(self.clone());
            t 
        }
        pub async fn ret_cls(f: impl Fn(String) -> String) -> impl Fn(String) -> String{
            let cls = |name: String|{
                String::from("")
            };
            cls
        }
    }
    let inst = Auth{};
    let get_trait = inst.get_trait();

}


// C must be send sync to be share between threads safely
impl<F: Interface + Clone, C: Send + Sync + 'static + Unpin + Sized + FnOnce() -> String> Interface for UserInfo<C, F>{}
struct UserInfo<C: Send + Sync + 'static, F: Clone> where 
    F: Interface, 
    C: FnOnce() -> String{
    data: F,
    __data: C,
    _data: Box<dyn Interface>,
}
impl<F: Interface + Clone, C: Send + Sync + 'static + Unpin + Sized + FnOnce() -> String> UserInfo<C, F>{
    fn set_data(cls: impl FnOnce() -> String, clstopass: C, f: F) -> impl Interface{
        
        struct ExecuteMe;
        struct MessageMe;
        trait ExecuteMeExt<A, B>{
            type Result;
        }
        impl ExecuteMeExt<MessageMe, String> for ExecuteMe 
            where String: Send, MessageMe: Send + Sync{
            type Result = MessageMe;
        }
        
        Self{
            data: f,
            __data: clstopass,
            _data: Box::new(
                ()
            ),
        }
    }
}

struct SizeableImage{
    size: (u16, u16)   
}
impl Into<SizeableImage> for String{
    fn into(self) -> SizeableImage { // self refers to the String cause we're implementing this for String
        let mut splitted_size = self.split(",");
        let width = splitted_size.next().unwrap();
        let height = splitted_size.next().unwrap();
        SizeableImage{
            size: (width.parse::<u16>().unwrap(), height.parse::<u16>().unwrap()) 
        }
    }
}
fn construct_image<VALUE>(size: VALUE) where VALUE: Into<SizeableImage>{}

struct ErrorHandler<E> where E: std::error::Error{
    cause: Box<dyn std::error::Error>, // any type could causes the error at runtime, Error trait is implemented for that
    err: E
}
#[derive(Debug)]
struct ErrorItself{}
impl std::error::Error for ErrorItself{}
impl std::fmt::Display for ErrorItself{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
// let err = ErrorHandler{
//     cause: Box::new(ErrorItself{}), // return the error on the heap to move it around to implement for other types
//     err: ErrorItself{}
// };


async fn ltg(){

    // C must be ?Sized since its size can't be known at compile time
    // its can be either &[] or any type
    struct Gene<'r, C: ?Sized>{ 
        pub chromosemes: &'r C,
    }

    let gene = Gene::<'_, [u8]>{
        chromosemes: &[0, 255]
    };
    
    impl std::fmt::Display for ClientError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    #[derive(Debug)]
    struct ClientError{}
    impl std::error::Error for ClientError{}
    let boxed_error: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(ClientError{});
    
    // traits
    //     - method param
    //     - return type
    //     - bound to generic and generic would be the type
    //     - cast the generic into a triat then bound the trait gat to other traits 
    //     - put them in box
    type ClsMe = Box<dyn FnOnce() -> ()>;
    trait NewTrait: Clone + FnOnce() -> (){} // if we want to implement NewTrait for the Fancy all the supertraits must be implemented for Fancy
    let cls = Box::new(||{});
    let cls_ = Box::pin(async move{}); // for future we must pin them
    struct Fancy<A> where A: Copy{name: ClsMe, age: A, fut: std::pin::Pin<Box<dyn futures::Future<Output=()>>>}
    let fancy = Fancy::<u8>{name: cls, age: 23, fut: cls_};
    impl<A: Copy> Fancy<A>{
        fn get_param(param: impl FnOnce() -> ()) -> impl Clone{
            String::from("") // we can return String in here since it implements Clone
        } 
    }

    #[derive(Debug)]
    struct CustomError{data: u8}
    impl std::fmt::Display for CustomError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    
    impl std::error::Error for CustomError{} // in order to return an instance of CustomError the Error trait must be implemented for it so we can return the instance inside a boxed type
    // ----------------------------------------------------
    //              boxing traits be like: 
    // ----------------------------------------------------
    // (putting them on the heap and return an smart pointer with valid lifetime to move the around safely as an object)
    let boxed_cls: Box<dyn FnOnce() -> () + Send + Sync + 'static> = Box::new(||{});
    // boxing the Error trait allows us to handle any possible runtime errors 
    // reason of putting the trait inside the box is because we don't know the 
    // exact type of the object that caused the error and by putting it inside 
    // the box we're converting it into a safe object cause traits are not sized
    // at compile time their size depends on the implementor at runtime smt that 
    // implements the trait, if we don't want to box it we can use it as the return
    // type of the method but it needs to be implemented for the exact object that
    // may causes the error at runtime and we should return an instance of that 
    // type which implements the Error trait already, with this approach there is
    // no need to put the trait inside the box cause we know the exact type of
    // object that may causes the error and the syntax be like => ... -> impl Error{}
    let boxed_err: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(CustomError{data: 0}); // the instance of the implementor must be passed  - boxing trait to move them around as an object 
    // to move the future objects around we should pin them (the mutable pointer) into the ram 
    // to prevent them from moving by the compiler at runtime sine we may want to solve them
    // in other scopes and threads hence we must have their previous location inside the ram
    // to put .await on them
    let boxed_fut: std::pin::Pin<Box<dyn futures::Future<Output=String>>> = Box::pin(async move{String::from("")}); 
    let mut pinned_boxed_fut = Box::pin(async move{}); // pinning the boxed future to move it around for future solvation
    { // now we can await on the future in other scopes
        // await on the mutable pointer of the future cause we want to await on pinned_boxed_fut in later scopes
        // we can do this cause we've pinned the boxed future (pointer to future) on the ram which allows us to 
        // move it safely between scopes and threads
        (&mut pinned_boxed_fut).await; 
    }
    pinned_boxed_fut.await; // solve the future itself


    type ActorCls = Box<dyn FnOnce(fn() -> String) -> ()>;
    type PinnedBoxedFut = std::pin::Pin<Box<dyn futures::Future<Output=String>>>; // pinning the boxed future will be used to move the future around other scopes cause they can't move safely and we must kinda convert them into an object to move them
    pub struct GenericActor<'p, ActorCls: Clone, B, F> 
        where ActorCls: Send + Sync + 'static, 
        B: FnMut() -> fn() -> (),
        F: futures::Future<Output=String>{
        pub actor_cls: ActorCls,
        pub cls: B,
        pub fut: F,
        pub pinned: PinnedBoxedFut, // we can solve this later by putting .await on pinned field
        pub db: std::pin::Pin<&'p mut HashMap<String, String>> // pinning the mutable pointer of the map into the ram to move it safely between scopes without having changes in its location by the compiler
    }
}

fn serding(){
    
    #[derive(Serialize, Deserialize, Debug)]
    struct DataBucket{data: String, age: i32}
    let instance = DataBucket{data: String::from("wildonion"), age: 27};
    ///// encoding
    let instance_bytes = serde_json::to_vec(&instance);
    let instance_json_string = serde_json::to_string_pretty(&instance);
    let instance_str = serde_json::to_string(&instance);
    let isntance_json_value = serde_json::to_value(&instance);
    let instance_json_bytes = serde_json::to_vec_pretty(&instance);
    let instance_hex = hex::encode(&instance_bytes.as_ref().unwrap());
    ///// decoding
    let instance_from_bytes = serde_json::from_slice::<DataBucket>(&instance_bytes.as_ref().unwrap());
    let instance_from_json_string = serde_json::from_str::<DataBucket>(&instance_json_string.unwrap());
    let instance_from_str = serde_json::from_str::<DataBucket>(&instance_str.unwrap());
    let isntance_from_json_value = serde_json::from_value::<DataBucket>(isntance_json_value.unwrap());
    let instance_from_hex = hex::decode(instance_hex.clone()).unwrap();
    let instance_from_hex_vector_using_serde = serde_json::from_slice::<DataBucket>(&instance_from_hex);
    let instance_from_hex_vector_using_stdstr = std::str::from_utf8(&instance_from_hex);
    let instance_from_vector_using_stdstr = std::str::from_utf8(&instance_bytes.as_ref().unwrap());
    
    println!(">>>>>>> instance_hex {:?}", instance_hex);
    println!(">>>>>>> instance_from_bytes {:?}", instance_from_bytes.as_ref().unwrap());
    println!(">>>>>>> instance_from_json_string {:?}", instance_from_json_string.unwrap());
    println!(">>>>>>> instance_from_str {:?}", instance_from_str.unwrap());
    println!(">>>>>>> isntance_from_json_value {:?}", isntance_from_json_value.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_serde {:?}", instance_from_hex_vector_using_serde.unwrap());
    println!(">>>>>>> instance_from_vector_using_stdstr {:?}", instance_from_vector_using_stdstr.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_stdstr {:?}", instance_from_hex_vector_using_stdstr.unwrap());

}

// convert any generic of Vec into a generic slice by leaking and consuming the 
// memory of the vector to return an static reference to the leacked part since 
// that part will never be freed until the lifetime of the app
pub fn vector_slice<T>(s: Vec<T>) -> &'static [T]{
    Box::leak(s.into_boxed_slice())
}


fn dynamic_typing(){

    /* 
        we can leverage the power of trait in rust to make a dynamic type and calls
        using Box<dyn Trait> since there might be some situation that we don't know 
        the exact type of some object the only thing it's worth nothing to know is 
        that it MUST implements the Trait so we could act like an object safe trait.
        Any trait is an object safe trait allows us to be Boxed and its methods get 
        dispatched dynamically at runtime on any type that implements the Any trait.
    */
    // dynamic dispatching and typing means this type can be of type anything it only needs
    // to implement the Any trait so we can cast its instances of the type into the actual 
    // concrete type, we can use the idea of dynamic dispatching to create dynamic typing
    // a type can be of Any if it implements the Any trait then we could cast it into the 
    // concrete type since Box<dyn Trait> in Rust is like type assertion in Go used for dynamic 
    // typing and dispatching make sure type implements Any trait in order to do the casting 
    // it's like type assertion in Go with interface{} to cast the object of type interface{} 
    // to its underlying concrete type the idea beind Any is the same as type assertion and
    // dynamic dispatching process or Box<dyn Trait> which allows us to box any type that 
    // implements Any trait so we can do the casting operation later and assert that the 
    // underlying type of Any is what we're trying to casting it into the desired concrete
    // type, generally we can use traits to add interfaces to structs, for dynamic typing 
    // dispatching and polymorphism using Box<dyn Trait>, casting trait object into the type,
    // using downcast_ref() method, casting type into the trait object using &String as &dyn Trait
    // the Box<dyn Trait> basically means that putting the trait behind an smart pointer on the
    // heap and since we don't know the exact type of implementor we add a dyn keyword behind 
    // the trait it's worth noghing to say that the implementor must implements the Trait in 
    // order to call trait methods on the instance or cast the trait into our concrete type.

    use std::any::{Any, TypeId}; // TypeId is a global unique id for a type in the whole app
    let boxed: Box<dyn Any> = Box::new(3_i32); // Any trait object, i32 implements Any trait

    let mut name: Box<dyn Any>; // name can be any type that implements the Any trait, later we can cast or assert the value into the desired type
    name = Box::new(String::from("wildonion"));
    println!("string before casting and mutating it : {:?}", name);
    println!("string address before casting and mutating it : {:p}", &name);

    let name_id = (&*name).type_id();
    // assert!(name_id, "{}", TypeId::of::<String>());

    // assert that the name type can be casted into the String or not
    // it's like type assertion in Go: 
    // if e, ok := name.(*string); ok{}
    match name.downcast_mut::<String>(){ // trying to cast the trait object into a mutable String type
        Some(mutable_ref) => {
            // it's mutable pointer we can change the content of the name
            // dereferencing won't change the address it only changes the underlying data (same address but different value)
            *mutable_ref = String::from("changed wildonion");
        },
        None => {
            println!("can't cast into string");
        }
    };

    // dereferencing won't change the address of name, it's the same as before
    println!("string after casting and mutating it : {:?}", name);
    println!("string address after casting and mutating it : {:p}", &name);

    struct Player{
        pub nickname: String,
    }
    let player = Player{nickname: String::from("")};
    let mut casted: Box<dyn std::any::Any> = Box::new(player);
    match casted.downcast_mut::<Player>(){
        Some(player) => {
            (*player).nickname = String::from("wildonion");
        },
        None => {}
    };

}


fn but_the_point_is1(){
    
    let fut = async move{};
    let pinned = Box::pin(fut);
    println!("fut pinned address {:p}", pinned);
    fn getFutPinned(fut: std::pin::Pin<Box<dyn std::future::Future<Output=()>>>){
        println!("[getFutPinned] - fut pinned address {:p}", fut);    
    }
    getFutPinned(pinned);
    
    let mut name = String::from("");
    // two different address
    let pname = &name;
    let pname1 = &name;
    // let mut pmutname = &mut name;
    let pinned = std::pin::Pin::new(Box::new(name.clone()));

    println!("name address with pname: {:p}", pname);
    println!("name address with pname1: {:p}", pname1);
    println!("name address with pinned: {:p}", pinned);
    
    move_pinned(pinned);
    fn move_pinned(pinned: std::pin::Pin<Box<String>>){
        println!("[move_pinned] - name address with pinned: {:p}", pinned);    
    }
    
    fn move_name(name: String){
        println!("[move_name] - name address : {:p}", &name);
    }
    
    fn move_name1(name: &String){
        println!("[move_name1] - name address : {:p}", name);
    }
    
    // pass by type or cloning or moving: every type in method has new owner thus new address
    move_name(name.clone());
    // pass by reference: type will have the same address as it's ousdie of the method
    move_name1(&name);
    
    
    
    let mut name = String::from("");
    let mut pname = &mut name;
    change_name(&mut name, &mut String::from("new value"));

    // we should use a same lifetime for each mutable pointer
    // cause we're updating name with new_name1 which requires 
    // same valid lifetime
    fn change_name<'valid_lifetime>(
        mut name: &'valid_lifetime mut String, 
        new_name1: &'valid_lifetime mut String
    ){
        // ------ same address different value
        println!("address before mutating: {:p}", name);
        *name = String::from("wildonion");
        println!("address after mutating: {:p}", name);
        // ------
        
        // ------ new address new value binding
        println!("address before new binding: {:p}", name);
        let mut new_name = String::from("new value");
        name = new_name1;
        println!("address after new binding: {:p}", name);
        // ------
        
        println!("final value of name: {}", name);
    }
    
}

fn but_the_point_is2(){

    // can't have immutable and mutable ref at the same time
    let mut init = String::from("wildonion");
    let mut init2 = String::from("wildonion");
    let pname = &init;
    let pname1 = &init;
    
    let mut mutpname = &mut init2; 
    println!("address of mutpname: {:p}", &mutpname);
    println!("mutpname points to init2: {:p}", mutpname);

    // same address but different value
    *mutpname = String::from("new wildonion");
    println!("value of init2: {}", init2);
    
    // new address and new value
    let mut new_binding = String::from("new wildonion1");
    mutpname = &mut new_binding;
    println!("address of mutpname: {:p}", &mutpname);
    println!("mutpname now points to new binding location: {:p}", mutpname);
    println!("value of mutpname: {}", mutpname);

    // same val different address
    // address of pname: 0x7ffd7875b570
    // address of pname1: 0x7ffd7875b578
    println!("address of pname: {:p}", &pname);
    println!("address of pname1: {:p}", &pname1);
    
    // both are pointing to the name
    // pname points to: 0x7ffd7875b558
    // pname1 points to: 0x7ffd7875b558
    println!("pname points to : {:p}", pname);
    println!("pname1 points to : {:p}", pname1);

}

fn but_the_point_is(){


    // rust often moves heap data around the ram for better allocation and 
    // optimisation like when a vector is growing at runtime it moves it in 
    // other locations and update all its pointers behind the scene.
    let mut vector = vec![1, 3, 5];
    
    println!("vec address: {:p}", &vector);
    
    // moving vector into a new scope with new ownership 
    // cause everthing must have only one owner
    fn get_vector_ownership(vec: Vec<u8>){
        
        // new address cause data will be moved into the function scopes after passing it
        println!("vec address in func: {:p}", &vec);
    }

    fn borrow_vector_ownership(vec: &Vec<u8>){
        
        // contains the same address of the outside vector
        // cause we've passed the vector by its pointer
        // and its point to the same location of the vector
        println!("vec address in func: {:p}", vec);
    }
    
    fn borrow_vector_ownership_mutable(vec: &mut Vec<u8>){
        
        vec.push(100);
        
        // contains the same address of the outside vector
        // cause we've passed the vector by its pointer
        // and its point to the same location of the vector
        println!("vec address in func: {:p}", vec);
    }
    
    // get_vector_ownership(vector);
    // borrow_vector_ownership(&vector);
    borrow_vector_ownership_mutable(&mut vector);

    // ********------********------********------********------********------
    // ********------********------********------********------********------
    // -> updating pointer with new binding changes both the address and value
    // -> dereferencing the pointer mutates the underlying value but keeps the address
    /* -> in Go: 
        u := User{Name: "onion"}
        p := &u
        println("p", p)

        // changing p completely with new address and value, this won't change the u
        p = &User{Name: "changed"} // changing address and value, breaks the pointer points to u

        // since p has been changed with new binding, dereferencing it will change 
        // the newly p value and it has nothing to do with u
        println("p", p)
        *p = User{Name: "wildonion"} // changing value keeps the address

        println("p", p)

        // this remains the same
        println("u", u.Name)
    */

    let mut me = String::from("onion");
    let mut mutme = &mut me;
    println!("me address : {:p}", mutme);
    
    // changing the address and the underlying value completely 
    // this logic break the pointer to the `me` and it replaces 
    // with a new address and value so after this the `me` value
    // won't change
    let mut new_me = String::from("changed");
    mutme = &mut new_me; // this won't change the `me`, breaks the pointer points to `me`
    println!("me address : {:p}", mutme);
    // -----> name is NOT changed after changing the mutme completely with a new binding
    
    // changing the underlying value only but NOT the address
    // this logic keeps the pointer pointing to the me location
    // and only mutate its underlying data
    *mutme = String::from("another changed");
    println!("me address : {:p}", mutme);
    // -----> name is changed after dereferencing the mutme

    // since `mutme` has been mutated with a new binding and address thus there is no
    // pointer points to the `me` location and dereferencing `mutme` (*mutme) will change 
    // the content inside the `mutme` only, which has been changed with "changed" and 
    // now has "another changed" value, this has nothing to do with `me` cause the `mutme` 
    // is not pointing to the `me` any more after getting a new location of new binding, 
    // the pointer breaks the pointing once a new binding put into it, if we didn't update 
    // the `mutme` with a new binding, after dereferencing it, the `me` would have upaded too.
    // ...  

    println!("me : {:?}", me); // onion as before
    println!("mutme : {:?}", mutme); // another changed
    // ********------********------********------********------********------
    // ********------********------********------********------********------

    type Ret = &'static str;
    fn add(num1: Ret, num2: Ret) -> Ret where Ret: Send{
        for ch in num2.chars(){
            num1.to_string().push(ch);
        }
        let static_str = Box::leak(num1.to_string().into_boxed_str()) ;
        static_str
    }

    let addfunc: fn(&'static str, &'static str) -> &'static str = add;
    let res = addfunc("wild", "onion");

    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
    // let name1: String; // this must be initialized
    // let pname = &name1; // ERROR: in Rust we don't have null or zero pointer it must points to some where in memory
    
    let mut name = String::from("wildonion"); // the pointee
    println!("name address : {:p}", &name);
    
    // NOTE: both pname and p0name have different addresses but pointes 
    // to the same underlygin address
    let pname = &name; // points to the location of name contains the name value
    let mut p0name = &name; // points to the location of name contains the name value
   
    println!("[SAME ADDR] pname pointer points to : {:p}", pname);
    println!("[SAME ADDR] p0name pointer points to : {:p}", p0name);
   
    // same address and same underlying data, cause pname points to the name address too
    p0name = pname;
    println!("[SAME ADDR] p0name pointer points to : {:p}", p0name);
    
    // however this is not safe in Rust to do this inside a function to change the
    // address a pointer points to but keeps the same underlying data cause literally 
    // chaning the address a pointer points to means that the underlying data must be 
    // changed cause pointers point to the address of a data and have their value.
    // if we want to do this we should use mutable pointer and rechange the pointer 
    // without dereferencing it
    let new_binding = &String::from("new onion"); 
    p0name = new_binding; // the actual name won't be changed cause this is not a mutable poitner
    println!("[CHANGED ADDR] p0name pointer points to : {:p}", p0name);
    
    let mut mutpname = &mut name; // points to the location of name contains the name value
    // changing underlying data same adderess by dereferencing 
    println!("mutpname pointer points to : {:p}", mutpname);
    *mutpname = String::from("wildonion"); // dereferencing it only update the undelrying data not the address
    println!("mutpname pointer points to : {:p}", mutpname); // mutpname still contains the address of the very first name variable but the value has changed
    
    // changing both address and underlying data by binding a new value
    let mut new_binding = String::from("onion"); 
    mutpname = &mut new_binding;
    println!("[CHANGED ADDR] mutpname pointer points to : {:p}", mutpname); // mutpname now contains completely a new value binding accordingly new location of the new binding
    // -----> name is NOT changed after changing the mutpname completely with a new binding
    // ....
    // pointers are immuatable by default 
    // {:p} requires the type implements AsRef trait, pointers implement this
    let name = String::from("wildonion");
    let pname = &name;
    println!("name address                       : {:p}", &name);
    println!("name address (pname)               : {:p}", pname);
    println!("address of pointer itself          : {:p}", &pname);
    println!("pname points to name address       : {:p}", pname);
    
    println!("------------------------------------");
    println!("--------same val | new addr---------");
    println!("------------------------------------");
    let mut pname1 = &String::from("oniontori");
    println!("pname1 points to new address       : {:p}", pname1);
    pname1 = pname;
    println!("pname1 now points to name address  : {:p}", pname1);
    println!("pname1 vale                        : {:?}", pname1);
    println!("pname1 address itself              : {:p}", &pname1);
    let new_binding = &String::from("layer1");
    pname1 = new_binding;
    println!("pname1 now points to a new address : {:p}", pname1);
    
    println!("------------------------------------");
    println!("--------same addr | new val---------");
    println!("--------new addr  | new val---------");
    println!("------------------------------------");
    let mut new_name = String::from("erfan");
    println!("new_name address                                 : {:p}", &new_name);
    println!("new_name value: {:?}", new_name);
    let mut pnamemut = &mut new_name;
    println!("pnamemut points to new_name address              : {:p}", pnamemut);
    println!(">>> dereferencing mutable pointer...");
    *pnamemut = String::from("completely new name");
    println!("new_name new value                               : {:?}", new_name); // also pnamemut has changed too
    let mut new_binding = String::from("new val");
    pnamemut = &mut new_binding;
    println!("pnamemut new value is                            : {:?}", pnamemut);
    println!("pnamemut now points to new address of new binding: {:p}", pnamemut);
    // pnamemut is no longer pointing to new_name
    // ...
    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------

    #[derive(Default, Debug, Clone)]
    struct User{
        name: String,
        age: u8,
    }

    let mut user = User::default(); // there is no null or zero pointer in rust thus the user must be initialized

    let mut mutpuser = &mut user; // mutating mutpuser mutates the user too
    println!("user address: {:p}", mutpuser); // contains user address
    println!("mutpuser address itself: {:p}", &mutpuser); // contains user address
    mut_user(mutpuser);

    fn mut_user(mut user: &mut User){ // passing by mutable pointer or ref to avoid moving

        // mutating the user pointer with new value which contains the user address
        // this makes an update to user instance too, can be viewable outside of the method
        println!("before mutating with pointer: {:#?}", user);
        user.name = "erfan".to_string(); // no need to dereference it since we're mutating only one field
        println!("after mutating with pointer: {:#?}", user);
        // or
        println!("before derefing: {:p}", user); // same as `contains user address`
        let mut binding = User{
            name: String::from("wildonion"),
            age: 0
        };
        // updating pointer which has the user instance value with a new binding by dereferencing pointer
        // note that we're not binding the new instance into the pointer completely cause by dereferencing
        // the underlying data will be changed
        *user = binding; // dereferencing the pointer to mutate it with new binding 
        println!("user after derefing: {:#?}", user);
        println!("user address after derefing: {:p}", user); // same as `contains user address`

    }

    // println!("out after mutating with pointer: {:#?}", user);
    let mut binding = User{
        name: String::from("wildonion"),
        age: 0
    };
    println!("mutpuser address itself: {:p}", &mutpuser); // contains user address
    println!("mutpuser contains address before binding: {:p}", mutpuser); // same as `contains user address`
    // binding a complete new instance to mutpuser, causes to point to new location
    mutpuser = &mut binding;
    // the address of mutpuser got changed and now points to new binding instance address
    println!("mutpuser contains address after binding: {:p}", mutpuser);
    println!("mutpuser address itself: {:p}", &mutpuser);

    // we're getting a mutable pointer to an in place User instance
    // the in place instance however will be dropped after initialization
    // and its ownership transferred into mutpuser, Rust won't allow us to
    // do so cause a pointer remains after dropping the in place instance
    // which is and invalid pointer, we must use a binding to create a longer
    // lifetime of the User instance then borrow it mutably
    // mutpuser = &mut User{
    //     name: String::from(""),
    //     age: 0
    // }; // ERROR: temporary value is freed at the end of this statement

    // SOLUTION: using a `let` binding to create a longer lived value
    // let binding = User{
    //     name: String::from("wildonion"),
    //     age: 0
    // };
    // *mutpuser = binding;


    // let puser = &user;
    // println!("user address (puser): {:p} ", puser); // contains the address of user
    // let anotherpuser = puser;

    // println!("user address (anotherpointer): {:p} ", anotherpuser); // also contains the address of user

    // println!("pointer address: {:p} ", &puser); // the address of the puser pointer itself
    // println!("anotherpointer address: {:p} ", &anotherpuser); // the address of the puser pointer itself

    // user address (puser): 0x7ffea5896328
    // user address (anotherpointer): 0x7ffea5896328
    // pointer address: 0x7ffea5896348
    // anotherpointer address: 0x7ffea5896390


    let users = (0..10)
        .into_iter()
        .map(|_|{
            User::default()
        })
        .collect::<Vec<User>>();
    let slice_is_faster = &users;
    fn get_users(users: &[User]) -> (&'static [User], Vec<User>){
        // lifetime of users ends up here in this function 
        // and can't be as static accordingly can't be return 
        // from function
        let users_vec = users.to_vec();
        let static_users = vector_slice(users_vec.clone());
        (static_users, users_vec)
    }

    trait Interface{
        type This;
        fn getName(&mut self) -> &Self;
    }
    #[derive(Debug, Default, Clone)]
    struct UserPl{
        Name: String,
        Age: u8,
        IsAdmin: bool,
    }
    impl Interface for UserPl{ // unlike Go Interface in Rust will be implemented for both pointer and none pointer instances
        type This = Self;
        // trait and interface methods
        fn getName(&mut self) -> &Self { // we can return ref since the pointer is valid as long as instance is valid
            if self.Name == "oniontori"{
                self.Name = String::from("wildonion");
            }
            self
        }
    }
    trait ObjectSafeTrait{}
    impl ObjectSafeTrait for (){}
    let mut user = UserPl{Name: String::from("oniontori"), Age: 28, IsAdmin: true};
    let trait_object: Box<dyn ObjectSafeTrait> = Box::new(());
    let mut mutpuser = &mut user;
    mutpuser.getName(); // mutating the Name field of the user instance using the Interface trait and its mutable pointer
    // println!("user is changed {:?}", user); // the user is changed using its mutable pointer
    mutpuser.Name = String::from("change to anything else again");
    println!("user is changed {:?}", user);
    // println!("mutpuser is changed {:?}", mutpuser); // the mutpuser is changed also

    type LargeUInt = u128;
    type Func<R, A = UserPl> = fn(A) -> R; // A has a default type set to UserPl
    let cls = |num: LargeUInt|{
        String::from("")
    };
    // `impl Trait` only allowed in function and inherent method argument 
    // and return types, not in variable bindings
    // let closure: impl Fn(u16) -> String = cls;

    #[derive(Default, Debug)]
    struct Player<'v, G: Default + Send + Sync + 'static, F> 
        where F: FnMut(Box<Player<G, F>>) 
            -> Result<
                std::pin::Pin<Box<dyn futures::Future<Output=&'v [u8]>>>, 
                Box<dyn std::error::Error + Send + Sync + 'static>> + Default{
        board: Vec<&'v [G]>,
        cls: F
    }

    trait UserExt<G, F>: Default{
        type Context;
        fn getCode() -> String;
    }
    impl<'valid, G: Default + Send + Sync + 'static, 
        F: FnMut(Box<Player<G, F>>) 
        -> Result<
            std::pin::Pin<Box<dyn futures::Future<Output=&'valid [u8]>>>, 
            Box<dyn std::error::Error + Send + Sync + 'static>> + Default
            > UserExt<G, F> for Player<'valid, G, F>{
        type Context = Player<'valid, G, F>;
        fn getCode() -> String {
            String::from("return value")
        }
    }
}

pub fn fillSchemaParser(){
    use std::collections::{BTreeMap, HashMap};
    use indexmap::IndexMap;

    // serde_json uses BTreeMap to decode the json string, in order the following 
    // logic works we should maintain the order of the keys in the order they are 
    // inserted into the map of the json string while we're decoding it. we should
    // enable the preserve_order feature in the serde_json crate. to keep the order
    // of the keys as exactly they're appearing in the json string
    let schema = String::from("server,2,1,3,4#");
    let json_str = r#"{"setip": "85.9.107.203", "setcode": "0", "setport": "7013", "setcode1": "0"}"#;
    let actions: serde_json::Value = serde_json::from_str(json_str).unwrap();

    // now serde json preserve the order of keys but BTreeMap doesn't due to building the
    // map based on the hash of each key, we're using IndexMap which maintain the state 
    // of the keys as they're inserted into the json string.
    let map = serde_json::from_value::<IndexMap<String, String>>(actions).unwrap();
    let values = map.values()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut splitted_chars: Vec<String> = schema.split_terminator(&[',', ':', '|', '.'][..])
        .map(|c| c.to_string())
        .collect();
    let mut spec_chars: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

    for i in 0..splitted_chars.len(){
        if splitted_chars[i].contains('#') || splitted_chars[i].contains('*'){
            spec_chars.insert(i, splitted_chars[i].clone());
        }
        let index = splitted_chars[i].trim_matches(|c| c == '#' || c == '*').parse::<usize>().unwrap_or(0); // don't use "" to char, char is inside '' 
        if index > 0 && index <= values.len(){ // make sure that the index isn't out of bound 
            splitted_chars[i] = values[index - 1].clone();
        }
    }

    for (key, val) in spec_chars{
        // inject the val at position key into the splitted_chars vector
        splitted_chars.insert(key, val);
    }

    // keeps the # or *, use hte spec_chars map to insert them
    println!("updated schema => {:?}", splitted_chars);

}