



use crate::*;



/* 
    
    --------------------------------------------------------------------
    ------------------- Ownership an Borrowing Recaps ------------------
    --------------------------------------------------------------------
    https://github.com/wildonion/cs-concepts?tab=readme-ov-file#-wikis
    https://github.com/wildonion/gvm/wiki/Ownership-and-Borrowing-Rules
    https://github.com/wildonion/rusty/blob/main/src/llu.rs
    https://github.com/wildonion/rusty/blob/a42b11dc96b40b059c60efa07513cdf4b93c5fab/src/ltg2.rs#L10
    https://github.com/wildonion/rusty/blob/a42b11dc96b40b059c60efa07513cdf4b93c5fab/src/ltg3.rs#L8
    https://github.com/wildonion/rusty/blob/a42b11dc96b40b059c60efa07513cdf4b93c5fab/src/ltg6.rs#L6
    https://www.reddit.com/r/rust/comments/dymc8f/selfreference_struct_how_to/
    https://arunanshub.hashnode.dev/self-referential-structs-in-rust#heading-pinlesstgreater-the-objects
    https://github.com/wildonion/rusty/blob/a42b11dc96b40b059c60efa07513cdf4b93c5fab/src/main.rs#L5
    https://stackoverflow.com/questions/72562870/how-can-i-write-a-self-referential-rust-struct-with-arc-and-bufreader


    In Rust, the behavior of dropping values and updating pointers after moving a value is governed by 
    the ownership and borrowing rules, which are enforced at compile time to prevent issues such as memory 
    leaks and dangling pointers, also due to the fact that rust doesn't have gc concepts, every type in 
    rust has its own lifetime and once it goes out of scope like moving it into other scopes and threads 
    followings happen:
        0) lifetime belongs to pointers in overall and it means that we're borrowing a type that must be
            valid as long as 'a lifetime is valid or we're borrowing the type that must be valid for 'a
        1) first note that if there is a pointer of a type it's better not to move the type at all
            instead pass its reference or its clone to methods and other scopes otherwise rust says
            something like "borrowed value does not live long enough" means that we have a reference 
            to nonexistent object cause the object gets moved, this situation is like returning an 
            instance of an struct from its method but at the same time we're using its reference, some
            how we must tell the rust that our our move will keep the reference so please don't drop it
            when the value is being returned
        2) in Rust, when a value goes out of scope, it is dropped. This happens at compile time, and the Rust 
            compiler inserts the necessary code to ensure that the value is dropped when it is no longer needed, 
            this is a deterministic process that occurs when the variable goes out of scope, and it is not tied 
            to runtime events
        3) we can't return a pointer from a method since the actual type is owned by the method and once
            the method gets executed it goes out of scope and its lifetime or the type itself gets dropped 
            from the ram so to avoid making dangling pointer rust doesn't allow to return it in the first 
            place unless we use 'static or a valid lifetime (borrow the type for the lifetime of 'valid) or 
            use the lifetime of self cause self is valid as long as the instance is valid
        4) cases that a type might be moved are passing a heap data type to a method or new scope, growing
            a vector at runtime or returning a type from a method in all these cases the actual type goes
            out of scope and gets dropped from the ram after moving and its value will go into another 
            type where the method is called or the type is being passed into a new scope thus its location 
            gets changed during execution
        5) once the type gets passed to the new scope, the newly scope takes the ownership of the type 
            and create a new location in the ram to put the moved value in the new location, so in rust 
            the location of values get changed during compilation process if they get passed into scopes
            and the old ownership gets dropped from the ram completely
        6) basically by moving data means the data string of the name variable will be moved into a new location 
            inside the heap cause its ownership has been changed and belongs to a new name variabel inside the method
            but the location inside the stack of the very first name variable won't be changed, if we don't want that
            happen we can pass the data string by reference into the method or clone, passing by reference doesn't 
            create a new location and take the ownership it just passing the data itself but clonning makes a new 
            type and put the value inside of it it's like having two different variables with same data
        7) when a value is moved, Rust updates the pointers and references to that value to ensure that they 
            are not left pointing to invalid or dangling memory. This is a key aspect of Rust's memory safety 
            guarantees, after a value is moved, any attempts to use pointers or references to the moved value 
            will result in a compilation error, preventing the creation of dangling pointers
        8) once the value of a type goes out of scope and took new ownership the lifetime of the old one 
            is no longer valid and gets dropped completely from the ram and can't be used after moving also 
            the value has new ownership which has 
            new addresss location inside the ram later rust updates all the pointers of the old ownership
            with this new address so they point to the right and newly location to avoid getting dangled
        9) if we move a type into a new scope regardless of the type is behind a pointer, rust updates
            its pointer to points to the right location after moving however the pointer is no longer
            accessible at runtime cause the type gets moved, the updating process can be taken place for 
            those types that are safe to be moved, which are almost all types except those ones that doesn't 
            implement Unpin, those are not safe to be moved and must be pin into the ram to avoid moving 
            completely, types like future objects and sef-referential types are not safe to be moved cause 
            as soon as move happens the pointer to them gets broken and invalidated hence rust won't update 
            the pointer of them to points to the right location after moving which doesn't allow to move 
            them in the first place 
        10) we can either pin the object into the stack or heap, Arc and Rc and Box puts the type into 
            heap but Box has a pin method which is simple to pin the Boxed value which is on the heap
            already into the ram in other cases we should pin the Arced or Rced into the ram
        11) rust can't update the pointer of self-ref types we can use Pin or Arc, Rc to break the cycle
            Pin takes the pointer of the type to pin its value into the ram avoid moving at all costs then
            we can use the pin instead of the type, we pass mutable pointer of the type to the Pin since 
            &mut _ access the underlying data of the object and pinning &mut _ is like pinning the direct
            value of the object (we can mutate a data by its &mut _)
        12) rust pointers are safe cause after moving a type (if it's implement Unpin means it's safe to be moved)
            the compiler updates the location of the pointer to point to the right location of the newly address of
            the moved value cause the ownership of the value has changed and all its pointers must gets updated to 
            point to the new location, this is not true about the raw pointers and rust won't update the location of 
            raw pointers to the new one when two value gets swapped or moved into another scope, they still point to
            the old value even after swapping, in rust we should use pin when the pointer of a type can't be updated
            by the rust compiler after it gets moved pinning allows us to pin the pointer of the type into the ram 
            and explicitly prevents the value from being moved, so the references to the value remain valid without 
            the risk of the value being relocated in memory by the rust compiler generally in cases such as self-refrential 
            types to break the cycle, future objects for later solvation and raw pointers, so we can pin the type 
            into the ram and don't allow to be moved at all and in the first place therefore by pinning a value using 
            Pin, you are essentially telling the Rust compiler that the value should not be moved in memory, and the 
            compiler enforces this constraint, this is particularly useful when dealing with self-referential structures 
            or when you need to ensure that references to the value remain valid by not allowing the type to be moved
        13) pin uses cases: handling raw pointers, self-refrential types and future objects 
            raw pointer swapping won't change the pointers pointees or pointer values it only swaps the contents
            which is not good since the first type pointer points to a location now which contains the content 
            of the second type after swapping and vice versa in other words rust won't update each pointer value
            based on the swapped values and there would be the same as before which causes to have undefined behaviours
            and dangling issues as rust don't care about updating the location of each pointer to point to the right
            location after moving to fix this we can pin each instance to tell rust make those objects immovable cause
            we don't want to invalidate any pointer of them, we're avoiding this by pinning each instance value using
            their pointer into the ram (usually heap using Box::pin()) so they can't be able to be moved cause by
            moving rust needs to update pointers to point the right location after moving but this is not true 
            about these none safe types and by swapping them two values along with their pointer are swapped
        conclusion: 
            types that are not safe to be moved (don't impl Unpin or are !Unpin) like self-refrential structs, 
            future objects, raw pointers are the types that unlike normal types rust compiler won't update their 
            pointer to point to the right location inside the memory (new address) after they get moved into other 
            scopes it's because of they kinda have infinite size at compile time or don't have specific size at 
            all so they invalidate their own references and break the moves, in order to fix this we should pin 
            their value into the ram (stack using std::pin::Pin or heap using Box::pin()) by passing their pointer 
            to the pin() method to tell the rust that don't move their values at all so their pointers can be valid
            across the scopes and threads but note that we can move the type after its value it gets pinned to the
            ram cause the use of Box::pin and Pin ensures that the self-referential pointers are correctly managed, 
            allowing the values to be safely moved and swapped without invalidating the references, means Box::pin, 
            it creates a pinned reference, ensuring that the data the reference points to will not be moved in memory, 
            preventing it from being invalidated:
            
            `let pinned = Box::pin(&name);` creates a pinned reference to the name string, 
            making sure that it won't be moved in memory, however, when we call `get_name(name)`, 
            we are actually moving the ownership of the name string into the get_name function, 
            which is allowed because the name variable is not used after that point, therefore,
            although pinned prevents the reference from being invalidated, it doesn't prevent the 
            owned value from being moved, later on we should use the pinned type instead of the 
            actual type cause the pinned type has a fixed memory location for the value thus has 
            a valid pointer which won't get dangled at all cause the value can't be moved by the 
            compiler at all even if rust wants to move them it can't since we're telling rust hey 
            u back off this is pinned! but its location and the address inside the ram will be the 
            same in all scopes, this is because the Pin type ensures that the references remain valid 
            even after the values are moved, in summary, pinning a value using Pin in Rust ensures 
            that the value is safe to use and reference, even if the value is moved by the compiler, 
            because the pointer to the value is pinned into a fixed memory location already

            let name = String::from("");
            let pinned = Box::pin(&name);
            let pname = &name;
            fn get_name(name: String){}
            get_name(name);

    _________________________________________________
    _> code snippet for ownership and borrowing rules
    -------------------------------------------------
    let name = String::from("");
    let p1name = &name;
    fn get_name(name: String){}
    get_name(name);
    let pname = &name;

    after the call to get_name(name), the ownership of the String data is moved into the get_name method, 
    and the name variable is no longer valid. The pname pointer cannot be created after the move because 
    the original value has been invalidated. The behavior you described is accurate: the pointer p1name 
    gets updated after the move, but no new pointers can be created to the moved value. This is a deliberate 
    design choice in Rust to prevent the creation of dangling pointers and ensure memory safety.

    rust moves types specially heap data ones around the ram by passing them
    into a function call or other scopes (unless we pass them by reference or
    clone them) to make the ram clean by removing extra spaces hence the value of 
    those types takes palce in a new location inside the ram (heap), compiler 
    it then updates their pointers to point to the right location (new one) 
    to avoid dangling issues, almost every type is safe to be moved like heap 
    data ones, but self-referential and future objects are not safe to be moved 
    cause rust won't update their pointer to point to the right location after 
    they get moved, as the result, they must be pinned to the ram to avoid moving 
    them at all due to the facts that if there is any pointer of these type exist 
    it won't get updated by the compiler to point to the right location after 
    moving, solution to this would be either pin the value of those types like 
    pinning their mutable pointer to avoid moving completely or put them inside 
    Arc,Rc,Mutex or RefCell to break the cycle of pointing to their instance, this 
    one is mostly used to store an instance of a structure as the field of the 
    struct itself like: 
    struct Struct{ pub data: Arc<Struct> } or struct Struct{ pub data: Rc<Struct> }

    in Rust, ownership is a key feature that ensures memory safety and prevents issues 
    like memory leaks and data races. The ownership system revolves around three rules:
        1 - Each value in Rust has a variable that's called its owner.
        2 - There can only be one owner at a time.
        3 - When the owner goes out of scope, the value is dropped.
    this system allows Rust to manage memory efficiently and avoid common pitfalls associated 
    with manual memory management.

    can’t move pointer inside a method to tokio spawn or return it from the method unless we make it 
    static or use the lifetime of self, cause the pointer is owned by the method
    data are moved by default when they gonna go into another scope, we can take a reference to them 
    and pass the reference but not the data itself cause it’s behind a pointer already and data behind 
    pointers can be moved, or we can clone them to prevent their ownership from moving.
    compiler moves data around the ram at runtime and change their location inside the stack like when 
    an element gets poped out of a vector rust clean the memory of the vector and shift each element's 
    location to where the empty space is located so there would be no extra space after, that's why 
    their pointers might get dangled if the type doesn't implement the Unpin trait, those types that 
    implements Unpin are safe to move around the ram by compiler cause the compiler takes care of their 
    pointers automatically so at runtime the pointer points to the right location of the type inside 
    the stack and if the type doesen't impelement the Unpin means it's not safe to be mvoed by the 
    compiler, to move it around other scopes safely we should pin the mutable pointer of the type into 
    the stack to tell rust that you shouldn't move this at all cause we will use its location in other 
    scopes later on, like pinning a future trait object for future solvation or await on its mutable
    pointer, take note of that once the lifetime of the type goes out of scope type will be dropped out 
    of the ram and removed completely, so the recaps are:
        - can't move the type around if it's behind a pointer, use the pointer instead
        - Rust compiler often moves values (heap data) around in memory, for example, if we pass an struct into 
            another function, it might get moved to a different memory address, or we might Box it and 
            put it on the heap or if the struct was in a Vec<MyStruct>, and we pushed more values in, 
            the Vec might outgrow its capacity and need to move its elements into a new, larger buffer.
        - When a value is moved or dropped Rust updates the references to that data to ensure that no 
            dangling pointers are created, this is achieved through the ownership and borrowing rules, 
            which are enforced at compile time.
        - Here are some scenarios in which values may be moved in memory by the rust compiler itself, 
            this is a fundamental aspect of Rust's ownership and borrowing system, and it is designed 
            to ensure memory safety and prevent issues such as data races and dangling pointers:
                0 - heap data types move by default to avoid allocating extra spaces in the ram
                1 - returning a value from a method: by returning the value from method the owner gets dropped out of the ram and is no longer accessible, the value however goes into a new location and gets a new ownership where the method return type is being stored
                2 - Passing a value to a function: When a value is passed to a function, it may be moved to a different memory address if the function takes ownership of the value.
                3 - Boxing a value and putting it on the heap: When a value is boxed using Box::new, it is moved to the heap, and the pointer to the boxed value is stored on the stack.
                4 - Growing a Vec beyond its capacity: When a Vec outgrows its capacity and needs to reallocate memory, the elements may be moved to a new, larger buffer in memory.
                5 - In each of these cases, the Rust compiler ensures that the ownership and borrowing rules are followed, and it updates references and pointers to the moved values to maintain memory safety.
    
    _________________________________________________
    _> code snippet for ownership and borrowing rules
    -------------------------------------------------
    let name = String::from("");
    let pname = &name;
    
    println!("location: {:p}", &name);
    println!("value is {:?}", name);
    
    fn get_name(name: String){ // name gets moved completely by the get_name method, so we can't access name after this call
        
        println!("location: {:p}", &name);
        println!("value is {:?}", name);
        
    }
    
    get_name(name);
    // same value but different location cause the ownership has been taken by the compiler:
    // location before moving into get_name: 0x7fff81e14150
    // location after moving inside get_name: 0x7fff81e141b0

    // can't access pname in here since it's moved and we can't use a pointer of a data which has been moved or
    // is not good to move a data if it's behind a pointer already, we should pass the name by reference to the
    // get_name() method or clone it so in order to be able to use panem later.
    // println!("pname : {:?}", pname);

    Here's a breakdown of what happens in above code snippet:

        1 - The name variable owns the String data.
        2 - The pname reference borrows the name data.
        3 - When get_name(name) is called, the ownership of the String data is transferred to the get_name method.
        4 - the newly name variable inside the method now has a new location inside the ram and the memory address of 
            the name String data on the heap does not change when it is passed to the get_name method, the ownership 
            transfer does not involve changing the memory address of the very first data on the heap.
        5 - After the call to get_name, the name variable is no longer valid, and any attempt to use it will result in a compilation error.
        6 - The pointer pname is still valid after the call to get_name because it is a reference to the original String data. 
            However, if you try to use pname to access the String data after it has been moved into the get_name method, 
            you will encounter a compilation error due to the borrow checker's rules.
        in Rust, the ownership system and borrowing rules ensure that memory safety is maintained, and the compiler 
        enforces these rules at compile time to prevent issues such as dangling pointers and data races: 
            - when a value is moved, the memory address of the data on the heap does not change as a result of the ownership 
                transfer, the ownership transfer involves updating the ownership information and ensuring that the original owner 
                is no longer valid. However, the actual memory address of the data on the heap remains the same.
            - When a value is moved, the ownership is transferred, but the data itself is not physically relocated in memory, 
                instead, the ownership information is updated to reflect the new owner, and the original owner is invalidated.
    
       ______________________
      |                      | 
     _↓___________    _______|______
    |   val = 1   |  |   p = 0xA1   |
    |-------------|  |--------------|
    |     0xA1    |  |      0xA2    |
     -------------    --------------

    the pointer field points to the val field in memory address A, 
    which contains a valid i32. All the pointers are valid, i.e. 
    they point to memory that does indeed encode a value of the 
    right type (in this case, an i32). But the Rust compiler often
    moves values around in memory. For example, if we pass this struct 
    into another function, it might get moved to a different memory 
    address. Or we might Box it and put it on the heap. or if this 
    struct was in a Vec<MyStruct>, and we pushed more values in, 
    the Vec might outgrow its capacity and need to move its elements 
    into a new, larger buffer.

           ____________________________________________________
          |                                                    |   
         _↓_____________________________     __________________|______
        |                               |   |   val = 1  |  p = 0xA1  |
        |-------------------------------|   |-------------------------|
        |     0xA1      |     0xA2      |   |   0xB1     |     0xB2   |
         -------------------------------     -------------------------

    When we move it, the struct's fields change their address, but not their 
    value. So the pointer field is still pointing at address A, but address 
    A now doesn't have a valid i32. The data that was there was moved to address 
    B, and some other value might have been written there instead! So now the 
    pointer is invalid. This is bad -- at best, invalid pointers cause crashes, 
    at worst they cause hackable vulnerabilities. We only want to allow memory-unsafe 
    behaviour in unsafe blocks, and we should be very careful to document this 
    type and tell users to update the pointers after moves.

    --------------------------------------------------------------
    ------------------- Box, Pin, Future recap -------------------
    --------------------------------------------------------------
    
    all Rust types fall into two categories:
        1 - Types that are safe to move around in memory. This is the default, the norm. For example, 
            this includes primitives like numbers, strings, bools, as well as structs or enums entirely 
            made of them. Most types fall into this category!
        2 - Self-referential types, which are not safe to move around in memory. These are pretty rare. 
            An example is the intrusive linked list inside some Tokio internals. Another example is most 
            types which implement Future and also borrow data, for reasons explained in the Rust async book.
    Types in category (1) are totally safe to move around in memory. You won't invalidate any pointers by 
    moving them around. But if you move a type in (2), then you invalidate pointers and can get undefined 
    behaviour, as we saw before.
    Any type in (1) implements a special auto trait called Unpin. but its meaning will become clear soon. 
    Again, most "normal" types implement Unpin, and because it's an auto trait (like Send or Sync or Sized1), 
    so you don't have to worry about implementing it yourself. If you're unsure if a type can be safely moved, 
    just check it on docs.rs and see if it impls Unpin!
    Types in (2) are creatively named !Unpin (the ! in a trait means "does not implement"). To use these types 
    safely, we can't use regular pointers for self-reference. Instead, we use special pointers that "pin" their 
    values into place, ensuring they can't be moved. This is exactly what the Pin type does.

    Pinning in Rust refers to the ability to ensure that a value is not moved in memory and 
    tell the compiler hey don't move this around the ram when i pass it through scopes this 
    is particularly important for asynchronous programming and working with types that contain 
    self-referential pointers like a pinned reference to the inner future. By "pinning" a value, 
    you prevent it from being moved, which is crucial for maintaining the integrity of self-referential 
    data structures, note that we can pin either the mutable, or immutable or the type itself 
    into the ram but if we pin the mutable we can't have immutable pointers later on and vice 
    versa but we can pin immutable pointer of the type and have other immutable pointers in 
    the scope, also if a data implements Unpin means it can't be pinned and is safe to be moved 
    and if a data doesn't implement Unpin or it's !Unpin means it can be pinned into the ram and 
    it's not safe to be moved around.

    by means the type is safe to be moved is rust will take care of solving any dangling pointer 
    issue later on by updating their pointer state to reflect the new location of them inside the
    ram but when we say a type is not safe to be moved means that rust won't take care of this 
    automatically and we should pin the type into ram to avoid it from moving completely.

    types that implement Unpin can be moved safely but those types likes futures and tratis that
    implements !Unpin are not safe to be moved and if we need them later to use them like solving
    a future object we must pin their mutable pointer into the ram to prevent them from moving so 
    we need Pin them to safely poll them or solve them using .await, by pinning the pointer of the 
    type we can tell the rust that hey don't move this type around the ram when the type wants to 
    be moved trait objects like closures are dynamically sized means they're stored on the heap in 
    order to act them as a separate object or type we need to either put them behind a pointer or 
    box them, this would be true about the futures cause they're traits too. boxing is the best way
    of passing them between different scopes since box is an smart pointer which puts the data
    on the heap and points to it with a valid lifetime so it's better to pass future objects as
    a boxed value.
    future objects must be pinned to the ram before they can be solved or polled the reason 
    of doing this is first of all they're trait objects and traits are dynamically sized means 
    they're size will be known at runtime second of all due to the fact that rust doesn’t have 
    gc which causes not to have a tracking reference counting process for a type at runtime, 
    because it’ll move the type if the type goes out of the scope hence in order to solve and 
    poll a future in other scopes later on, we should pin it to the ram first which can be done 
    once we await on the future but if we want to solve and poll a mutable reference of a future 
    we should stick and pin it to the ram manually, first by pinning the future into the ram using 
    Box::pin, tokio::pin!(), std::pin::pin!() then do an await on another instance of future or the 
    mutable reference of the future object, so if it is required to call .await on a &mut _ reference, 
    cause .await consumes the object itself and we can't have it later so in this case the caller 
    is responsible for pinning the future by pinning future objects manually we make them as a safe 
    object before polling them like having a mutable reference to them or move them into other parts 
    to solve them in different parts.
    conclusion:
    so pinning logic must be used if a type is not safe to be moved (!Unpin) like future objects 
    and we want to move it safely without changing its location in the ram for future usage, which
    can be done by pinning the mutable pointer of the type into the ram, for future and trait based
    objects this can be done by pinning their box smart pointer with Box::pin or the type itself 
    with tokio::pin!(), std::pin::pin!() or std::pin::Pin::new(&mut Data{});
    recap:
    futures are trait objects and traits are dynamically sized and they must be behind pointer like 
    &dyn or Box<dyn also they're unsafe to be moved and must be first stick into the ram then we can 
    move them between different scopes, the process can be done by pinning the mutable pointer of the 
    type into the ram to prevent that from moving around by the compiler it's ok to put .await on the 
    fut without manual pinning cause .await do this but it consumes the future and we can't do whatever 
    we want with the future after that like if we want to await on another instance of the future like
    the mutable pointer of the future we must do the pinning process manually, like pin the future into 
    the ram first then await on its mutable pointer, in the first place futures are unsafe to be moved
    and they may gets moved by the compiler before getting polled so in order to use their reference 
    we should tell the compiler that i'm using the pointer of this future so don't move it around until
    i await on its mutable pointer well the compiler says you must pin it manually!
    the reason of pinning the mutable pointer of the object instead of its immutable pointer into the stack
    is because mutable pointer can access to the underlying data of the object and by mutating it we can 
    mutate the actual content and data of the object itself thus by pinning the mutable pointer into the 
    stack we're pinning the object itself actually and prevent it from moving around.
    types that implements Unpin means they can be unpinned from the stack later but types that are !Unpin 
    means they don't implement Unpin so can't be unpinned so are not safe to be moved and they must be 
    pinned to the ram.

    some objects are not safe to be moved around, between threads and scopes their value must be first pin 
    into the ram to make them safe for moving this can be done via std::pin::Pin::new(&mut Data{}); 
    as we can see above the mutable pointer of the object gets pinned into the ram so we can move it around 
    safely, reason of pinning the mutable pointer is because the mutable pointer has access to the underlying 
    data and its value and by pinning it we're actually pinning the object itself. in case of trait objects,
    actually traits are not sized at compile time and due to the fact that they're dynamically sized and stored 
    on the heap they must be in form of pointer like &'validlifetime dyn Trait or Box<dyn Trait> so pinning 
    their pointer be like Box::pin(trait_object); which allows us to move them safely as an object between 
    threads and other scopes without changing their location at runtime by the compiler, in case of future 
    objects they're trait objects too and they're not safe to be moved around, to do so we must pin them into 
    the ram first cause we might want to solve and poll them later in other scopes, when we want to solve and 
    poll a future we put .await after calling it, .await first consumes the future object and do the pinning 
    process for us but if we want to move the future manually between scopes we should pin its mutable pointer 
    manually then move the pinned object safely for future solvation like: Box::pin(async move{}); which pins 
    the pointer of the future object into the ram, in this case its better to put the future object into heap 
    using Box to avoid overflow issues and pin the Box pointer into the ram for future pollings.

    pinning the pointer of future object into the ram, future objects are traits and traits must be behind &dyn 
    or Box<dyn to be as an object at runtime thus we're pinning the box pointer of the future object which is 
    on the heap into the ram to avoid moving it for futuer solvation. in order to move the future between 
    different scopes safely we should first pin it into the ram then we can move it as an object between threads 
    and once we get into our desired thread we can put an await on the pinned boxed to solve the future reason 
    of doing so is because future objects are not safe to move around by the compiler and the must be pinned 
    first then move around, this behaviour is actually being used in tokio::spawn tokio will move the pinned 
    box of the future into its threads for future solvation also the future task and its output must be Send and 
    Sync, in order to avoid overflowing, pinning must be done by pinning the pointer of the future object and 
    since futures are dynamically sized their pointer will be a Box which is an smart pointer with a valid 
    lifetime, which store the data on the heap and returns a pointer to that

    self-referential structure is a type has fields which store references to the struct itself
    it would break as soon as the move happens and would invalidate it; the references would be 
    dangling and rust can't update the pointer to points to the new location of the type (Pin is better) 
    a solution to this is either using Arc Mutex for multithreaded or Rc RefCell in single threaded to break 
    the cycle when a field is pointing to the struct itself (see graph.rs) or using Pin, unsafe 
    or raw pointers so we go for the second option thus our recap on pin, pointer, ownership and
    borrowing rules would be:
    in rust, data (heap data) often move around the ram by the compiler to avoid allocating extra spaces
    at runtime, most of the data like heap data are safe to be moved means that they're 
    implementing Unpin trait which means they don't need to be pinned to prevent them from moving 
    cause once they get moved rust compiler can take care of their pointers to point to the right 
    and newly location of them to avoid having any dangling pointers after moving. those types that 
    are not safe to be moved are the one who don't implement Unpin trait and are !Unpin means 
    they can't be moved safely and their pointers must get pinned into the ram so their value can't 
    be moved into a new ownership variable and thus we can move them safely around the ram, the 
    reason that we can't move them it's because rust won't take care of updating their pointers at 
    compile time and we may have dangled pointers after moving them, these types can be future objects 
    and self-referential types which need to be used later in other scopes like solving a future object 
    or avoid a self-referential type from moving by pinning it into the ram cause self-referential 
    structs are a no-go, rust has no way of updating the address in the references if the struct is 
    moved since moving is always a simple bit copy in other words rust compiler can't update their 
    pointers to point to the right location which forces us to pin the type to not allow to be moved 
    at all cause they are inherently unsafe since they are implicitly invalidated if they are ever 
    moved we're not allowed to move them at all unless use Pin or break cycle using Arc,Rc,Mutex,RefCell

*/

async fn pinned_box_ownership_borrowing(){

    // ====================================
    //          Boxing traits
    // ====================================
    impl std::fmt::Display for ClientError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
            todo!()
        }
    }
    #[derive(Debug)]
    struct ClientError{}
    fn set_number() -> i32{ 0 }
    impl std::error::Error for ClientError{}
    let boxed_error: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(ClientError{}); // we can return the boxed_error as the error part 
    let boxed_cls: Box<dyn FnMut(fn() -> i32) -> ClientError + Send + Sync + 'static> = 
        Box::new(|set_number|{
            ClientError{}
        }); 

    // ====================================
    //          self ref type
    // ====================================
    // can't have self ref types directly they should be behind some kinda pointer to be stored on the heap like:
    // we should insert some indirection (e.g., a `Box`, `Rc`, `Arc`, or `&`) to break the cycle
    // also as you know Rust moves heap data (traits, vec, string, structure with these fields, ?Sized types) to clean the ram 
    // so put them inside Box, Rc, Arc send them on the heap to avoid lifetime, invalidate pointer and overflow issue
    // also Arc and Rc allow the type to be clonned
    type Fut<'s> = std::pin::Pin<Box<dyn futures::Future<Output=SelfRef<'s>> + Send + Sync + 'static>>;
    struct SelfRef<'s>{
        pub instance_arc: std::sync::Arc<SelfRef<'s>>, // borrow and is safe to be shared between threads
        pub instance_rc: std::rc::Rc<SelfRef<'s>>, // borrow only in single thread 
        pub instance_box: Box<SelfRef<'s>>, // put it on the heap to make a larger space behind box pointer
        pub instance_ref: &'s SelfRef<'s>, // put it behind a valid pointer it's like taking a reference to the struct to break the cycle
        pub fut_: Fut<'s> // future objects as separate type must be pinned
    }

    let mut future = async move{};
    tokio::pin!(future); // first we must pin the mutable pointer of the future object into the stack before solving/polling and awaiting its mutable pointer 
    (&mut future).await; 
    
    let fut = async move{};
    let mut pinned_box = Box::pin(fut); // in cases if we need to access the pinned value outside of the current scope cause the future is boxed and we can move it as an object
    (&mut pinned_box).await;
    pinned_box.await;

    /*
        the type that is being used in solving future must be valid across .awaits, 
        because future objects will be pinned into the ram to be solved later, worth
        to know that trait pointers are Boxes and we pin their pointer into ram like: 
        Pin<Box<dyn Future<Output=String>>>
    */

    fn get_data<G>(param: impl FnMut() -> ()) -> impl FnMut() 
        -> std::pin::Pin<Box<dyn std::future::Future<Output=String> + Send + Sync + 'static>>
        where G: Send + Sync + 'static + Sized + Unpin{ // G is bounded to Unpin means it can't be pinned into the ram
        ||{
            Box::pin(async move{
                String::from("")
            })
        }
    }

    async fn callback() -> i32 {3}
    // we can't add let func: fn callback() -> impl Future<Output = i32> but compiler can
    let callbackfunc = callback;
    callbackfunc().await;

    let pinned_boxed_future: std::pin::Pin<Box<dyn std::future::Future<Output=String>>> = 
        Box::pin(async move{
            String::from("")
        });

    async fn func(){}
    type Type = Box<dyn std::future::Future<Output=()> + Send + Sync + 'static>;
    struct Generic<'lifetmie, Type>{
        pub data: &'lifetmie mut Type // mutating mutable pointer mutates the underlying data too
    }
    let mut instance = Generic{
        /*  
            to have future objects as a type which are of type Future trait we have to
            put them behind a pointer and pin the pointer into the ram to get their result
            in later scopes by awaiting on them which actually will unpin their pointer,
            we can't use Box::new(async move{()}) if we want to access the result of the 
            future outside of the Boxed scope to solve this we must pin the boxed value 
            which in this case is pinning the pointer to the Future trait, and put an await
            on that in later scopes to unpin the boxed value from the ram to get the result
            of the future object

            since Future trait doesn't implement Unpin trait thus we can pin the boxed 
            type into the ram by constructing a new Pin<Box<Type>>. then Type will be 
            pinned in memory and unable to be moved.
        */
        data: &mut Box::pin(func()) // passing the result of calling async func to the pinned box
    };
    let unpinned_boxed = instance.data.await;
    /*  
        moving type can also be dereferencing the type which converts
        the pointer into the owned value but based on the fact that 
        if the type is behind a pointer we can't move it! so we can't
        deref the pinned boxed in here, we must clone it or borrow it 
        which clone is not working in here because Clone it's not 
        implemented for &mut Type which is the type of data field
    */
    // let deref_boxed = *instance.data;
    instance.data = &mut Box::pin(func()); // passing the result of calling async func to the pinned box
    

}