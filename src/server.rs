// server to listen for commands from client threads

mod search;
use std::fs;
use std::str;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::net::{TcpListener, TcpStream};
use std::net::{SocketAddr};
use std::thread;
use std::ops::{Bound, RangeBounds};

use std::path::Path;
use std::io::{Read, Write, Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate file_lock;
use file_lock::FileLock;


// listen for messages from a client

fn connection_thread(mut stream: TcpStream) -> Result<(), Error> {
	println!("Received a connection from: {}", 
	stream.peer_addr()?);	// network address of client unwrap to OK if
	let mut buffer = [0; 1024]; 	// zero buffer
      //let mut is_authenticated=false;
      //let mut is_guest=false;
      let mut username=String::from("");
      match fs::create_dir_all("users_server/") {
            Err(why) => {
                  println!("! {:?}", why.kind());
                  println!("Unable to create users_server directory: {}", why);
                  return Err(why);
              
                  


            },
            Ok(_) => {
                  println!("users_server directory is created.");
            }
      }	
      
    
      
	loop {
		let bytes_read =match stream.read(&mut buffer){
                  Ok(bytes_read) => (bytes_read),
                  Err(err)=> {
                     println!("Stream error, unable to read streadm: {}", err);
                     return Err(err);
                  }
            }; // unwrap to Ok if read from stream successful
		if bytes_read == 0 {	// no more to read
			return Ok(());
			
		}
		
		let cmd_line = String::from_utf8_lossy(&buffer[0..bytes_read]);
		//println!("cmd_line from buffer: {}", cmd_line);
		
		// split the command line into tokens: tokens[0] .. tokens[params-1]
		
		let tokens: Vec<&str> = cmd_line.trim().split_whitespace().collect();
            println!("command is: {}",cmd_line);
    		let mut args: Vec<String> = cmd_line.trim().split_whitespace().map(str::to_string).collect();

		match tokens[0] {
		
			// search -f file -s search_text
			
			"search" 
			//if is_authenticated && 
				if args.len() > 4 && args[1] == "-f" && args.iter().any(|i| i=="-s") => 
				{			
			
                              println!("SEARCH -f HERE !!!");
                              	
                              	
                              
                              let mut path = PathBuf::new();
                        	path.push("./users_server/");
                        	path.push(&username);
                            
                     
                              let file_name = &args[2];
                                    
                              path.push(file_name);
                                   
                              let search_text_args: Vec<_> = args.drain(4..).collect();
                              let search_text = search_text_args.join(" ");

                              //let mut file=Arc::new(Mutex::new(fs::File::open(path))).lock().unwrap();
                                 
                                          
                              let  mut contents= write_file_to_string(&path);
      
                              if contents=="Problem opening the file"{
                                 	contents.push('\r');
      				      match write!(stream, "{}", &contents){
                                                        Ok(_) => (),
                                                  Err(err) => {
                                                        println!("Unable to send command to client: {}", err);
                                                        return Err(err);
                                                   }
                                                               
                                              }
                                             println!("{}",contents);
                                       }else{
                                               
                                             let mut response = search::search_f(&contents, &search_text);
      
                                             
                                             println!("Response from search_f: {}", response);
                                             response.push('\r');
                                             //write!(stream, "{}", &response).unwrap();
                                             match write!(stream, "{}", &response){
                                                  Ok(_) => (),
                                                  Err(err) => {
                                                        println!("Unable to send command to client: {}", err);
                                                        return Err(err);
                                                   }
                                                               
                                              }
                                             
                                               
                                          }
      
                                     },   // end search -f file -s search_text 
                                       
                                 
                                   
                                   // search -s search_text
                                   
                                   "search" 
                                 //  	if is_authenticated && 
                                   	if args.len() > 2 && args[1] == "-s" => {
                                          println!("search -s found\n");
                                      
                                           	let mut path = PathBuf::new();
                              		path.push("./users_server/");
                              		path.push(&username);
                                  		
                                                let search_text_args: Vec<_> = args.drain(2..).collect();
                                                let search_text = search_text_args.join(" ");
                      
                                                //let mut path_string=String::from("./users_server/irfan/");
         
      
      
                                                //let r_directory=fs::read_dir(path2).unwrap();
                                                let r_directory=fs::read_dir(path);
                                                let r_directory=match r_directory {
                                                 Err(e) =>{
                                                      eprintln!("Path problem in 'search' command cannot find the directory to search files inside :{:?}", e);
                                                      return Err(e)
                                                      } 
                                                  Ok(r_directory) => (r_directory)
                                                 };
                                                 let mut answer=String::from("");
                                                 for entry_res in r_directory.filter_map(Result::ok){
            
                                                      let entry = entry_res;
                                            
                                                      //if entry.is_err() { continue; }
                                            
                                                      // let entry=match entry{
                                                      //     _ =>{
                                                      //         eprintln!("error reading directory");
                                                      //         break;
                                                      //     } ,
                                                      //     Ok(entry) => entry
                                                      // };
                                            
                                                      let this_file_name_buf = entry.file_name();
                                                      let this_file_name = this_file_name_buf.to_str();
                                            
                                                      let this_file_name=match this_file_name{
                                                          None =>{
                                                              eprintln!("error in search -s command invalid this_file_name variable");
                                                              break
                                                          } ,
                                                          Some(this_file_name)=> this_file_name.to_string()
                                                      };
                                                     
                        
                                                     let  contents= write_file_to_string(&entry.path());
      
                                                     if contents=="Problem opening the file"{
            
                                                      println!("{}",contents);
                                                      }else{
                                                         
                                                            let mut response =  search::search_s(&contents, &search_text, &this_file_name);
                                                            //println!("Response from search_s: {}", response);
                                                            
                                                           println!("pushing::{}\n", response);
                                                           // answer.push_str(response.as_str().trim());
                                                           answer.push_str(response.as_str());
                                                           
                                                     }
                                                   
                                 
      
                                          }
                                          println!("Response from search_s: {}", answer);
                                          //write!(stream, "{}", &answer).unwrap();
      					   // answer.push('\n');
      					   answer.push('\r');
      
                                          match write!(stream, "{}", &answer){
                                                Ok(_) => (),
                                                Err(err) => {
                                                      println!("Unable to send command to client in search -s command: {}", err);
                                                      return Err(err);
                                                }
                                                         
                                          }
                                          
                          //          }
                           //   }

                      //  }else{
                      //        println!("only authenticated users can use search functionality");
                       // }



        			
			}, // end search -s search_text

                  "write" if args.len() > 2 && args[1] == "-a" => {
                        println!("write -a::user command: {}",cmd_line);

                        //set file path
                        let mut path = PathBuf::new();
                        path.push("./users_server/");
                        path.push(&username);
                        path.push(&args[2]);
                        
                        let should_we_block  = true;
			 let lock_for_writing = true;
                   let path_to_str=match path.as_path().to_str(){
                        Some(tt) => (tt),
                        None=> {
                           println!("No PATH:invalid path in write -a command");  // it will throw error below anyway without panicking
                           continue
                           
                        }
                  };

    			 let filelock = match FileLock::lock(path_to_str, 	should_we_block, lock_for_writing) {
        			Ok(lock) => lock,
        			Err(err) => 
        				
        				return Err(err),
    			};

                        let contents = write_file_to_string(&path);

                        if contents == "Problem opening the file"{
                              match write!(stream, "{}", &"Problem finding file\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client in write -a command when sending contents: {}", err);
                                          return Err(err);
                                    }
                              }
                        }
                        else{

                              //open file with append options
                              let mut file = match OpenOptions::new()
                              .write(true)
                              .append(true)
                              .open(&path){
                                    Err(err) =>{
                                          eprintln!("error opening file for write -a mode {}",err);
                                          return Err(err);
                                    } ,
                                    Ok(file) => (file)
                              };

                      

                              //ask client for text to append
                              match write!(stream, "{}", &"Enter text to append to end of file\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client during requesting text to be appended in write -a mode: {}", err);
                                         return Err(err);
                                    }
                              }

                              //read text from client
                              let mut reader = BufReader::new(&stream);
                              let mut msg = String::from("");

				      match reader.read_line(&mut msg) {
					      Ok(_) => (),
					      Err(err) => {
					      	println!("Unable to read into buffer in write -a during reader: {}", err);
					      }
                              }
                              msg.pop();
                              println!("From client: {}", msg);

                              //write to file, append only
                              if let Err(e) = write!(file, "{}", &msg) {
                                    eprintln!("Couldn't write to file in write -a command: {}", e);
                                }

                        }

			filelock.unlock();
                  }, // end write -a

                  "write" if args.len() > 2 && args[1] == "-n" => {
                        println!("write -n::user command: {}",cmd_line);

                        //set file path
                        let mut path = PathBuf::new();
                        path.push("./users_server/");
                        path.push(&username);
                        path.push(&args[2]);

                        let should_we_block  = true;
			 let lock_for_writing = true;


                   let path_to_str=match path.as_path().to_str(){
                        Some(tt) => (tt),
                        None=> {
                           println!("in write -n command:invalid path this may be users_server folder is not there"); // it will throw error below anyway without panicking
                           continue
                           
                        }
                  };

    			 let filelock = match FileLock::lock(path_to_str, 	should_we_block, lock_for_writing) {
        			Ok(lock) => lock,
        			Err(err) => 
        				return Err(err),
    			};


                        let mut contents = write_file_to_string(&path);

                        if contents == "Problem opening the file"{
                              match write!(stream, "{}", &"Problem finding file\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client in write -n command: {}", err);
                                          return Err(err);
                                    }
                              }
                        }
                        else{

                              //open file
                              let mut file =match OpenOptions::new()
                              .write(true)
                              //.open(path)
                              //.//unwrap();
                              .open(&path){
                                    Err(err) =>{
                                          eprintln!("error opening file for write -n command {}",err);
                                          return Err(err);
                                    } ,
                                    Ok(file) => (file)
                              };
                              
                              
                           

                       

                              //ask client for text
                              match write!(stream, "{}", &"Enter text to overwrite\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client during sending 'Enter text to overwrite' in write -f: {}", err);
                                         return Err(err);
                                    }
                              }

                              //read text from client
                              let mut reader = BufReader::new(&stream);
                              let mut msg = String::from("");

				      match reader.read_line(&mut msg) {
					      Ok(_) => (),
					      Err(err) => {
					      	println!("Unable to read into buffer in write -n command: {}", err);
					      }
                              }
                              msg.pop();
                              println!("From client: {}", msg);

                              //write to file, overwrite text
                              if let Err(e) = file.seek(SeekFrom::Start(0)){
                                    eprintln!("Couldn't seek in file in write -n command: {}", e);
                              }
                              if let Err(e) = write!(file, "{}", &msg) {
                                    eprintln!("Couldn't write to file in write -n command: {}", e);
                              }


                        }
			filelock.unlock();
                  }, // end write -n

                  "write" if args.len() > 2 && args[1] == "-f" => {
                        println!("write -f:: user command: {}",cmd_line);

                        //set file path
                        let mut path = PathBuf::new();
                        path.push("./users_server/");
                        path.push(&username);
                        path.push(&args[2]);

			 let should_we_block  = true;
			 let lock_for_writing = true;

                   let path_to_str=match path.as_path().to_str(){
                        Some(tt) => (tt),
                        None=> {
                           println!("path problem in write -f command"); // it will throw error below anyway without panicking
                           continue
                           
                        }
                  };

    			 let filelock = match FileLock::lock(path_to_str, 	should_we_block, lock_for_writing) {
        			Ok(lock) => lock,
        			Err(err) => 
        			
        				return Err(err),
    			};


                        let contents = write_file_to_string(&path);

                        if contents == "Problem opening the file"{
                              match write!(stream, "{}", &"Problem finding file\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to server in write -f command during sending sentence:'Problem opening the file' : {}", err);
                                          return Err(err);
                                    }
                              }
                        }
                        else{

                        

                              //ask client for text
                              match write!(stream, "{}", &"Enter text to prepend\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client in write -f command: {}", err);
                                         return Err(err);
                                    }
                              }

                              //read text from client
                              let mut reader = BufReader::new(&stream);
                              let mut msg = String::from("");

				      match reader.read_line(&mut msg) {
					      Ok(_) => (),
					      Err(err) => {
					      	println!("Unable to read into buffer in write -f command: {}", err);
					      }
                              }
                              msg.pop();
                              println!("From client: {}", msg);

                              //append file contents to data received from client
                              msg.push_str(&contents);

                              //delete destination file
                              fs::remove_file(&path)?;

                              //revive destination file
                              let mut temp_file = File::create(&path)?;

                              //write properly formatted data to destination file
                              temp_file.write_all(&msg.as_bytes())?;
                        }
			filelock.unlock();
                  }, // end write -f

                  "create" => {
                        println!("user command: {}",cmd_line);
                        let mut msg = String::new();
                        //let mut already_created:i8=0;
                        let mut user_path=String::from("users_server/");

                        user_path.push_str(tokens[2]);

                        if Path::new(&user_path).exists(){
                              //already_created=1;
                             //user exists

                              match write!(stream, "{}", &"User is already created. Please try again.\n"){
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to send command to client in create user command: {}", err);
                                    //return Err(err);
                              }
                              }
                        }else{
                            
                              //create user here
                              match fs::create_dir(&user_path) {
                                    Err(why) => {
                                          println!("! {:?}", why.kind());

                                          match write!(stream, "{}", &"Error creating user dir..\n"){
                                                Ok(_) => (),
                                                Err(err) => {
                                                      println!("Unable to send command to client in create user command: {}", err);
                                                      return Err(err);
                                                }
                                          }


                                    },
                                    Ok(_) => {
                                         
                                          //user_path+"/txt.pub"
                                          //user_path+"/txt.encrypt"
                                          let mut user_pub_file = File::create([&mut user_path, "/txt.pub"].join(""))?;
                                          let mut user_enc_file = File::create([&mut user_path, "/txt.encrypt"].join(""))?;

                                          match write!(stream, "{}", &"request public key\n"){
                                                Ok(_) => {
                                                      println!("requested public key");
                                                      ()
                                                },
                                                Err(err) => {
                                                      println!("Unable to send command to client during requesting public key: {}", err);
                                                      return Err(err);
                                                }
                                          }
                                          let mut reader = BufReader::new(&stream);
                                          match reader.read_line(&mut msg) {
								Ok(_) => (),
								Err(err) => {
									println!("Unable to read into buffer in create user command: {}", err);
									return Err(err);
								}
								   
							     }

                                          user_pub_file.write_all(msg.trim().as_bytes())?;



                                          match write!(stream, "{}", &"request encrypted\n"){
                                                Ok(_) => {
                                                      println!("requested encrypted");
                                                      ()
                                                }
                                                ,
                                                Err(err) => {
                                                      println!("Unable to send command to client during requesting encrypted: {}", err);
                                                      return Err(err);
                                                }
                                          }
                                          let mut reader = BufReader::new(&stream);
                                          let mut msg2=String::from("");
                                          match reader.read_line(&mut msg2) {
								Ok(_) => (),
								Err(err) => {
									println!("Unable to read into buffer in create user command: {}", err);
									return Err(err);
								}
								   
							     }
                                          
                                          user_enc_file.write_all(msg2.trim().as_bytes())?;

                                          


                                          match write!(stream, "{}", &"User creation successful. Please login\n"){
                                                Ok(_) => {
                                                      println!("user creation successful");
                                                      ()
                                                }
                                                ,
                                                Err(err) => {
                                                      println!("Unable to send command to client in create user command: {}", err);
                                                      return Err(err);
                                                }
                                          }

                                          //user_enc_file.write_all(b"Hello, world!")?;



                                    },
                                }


                       }
                       

                  },  // end create

                  "send" if args.len() > 1 => {
                        //specify path to user's folder
                        let mut path = PathBuf::new();
                        path.push("./users_server/");
             		path.push(&username);
                        
                        //get filename
                        let file_path: Vec<&str> = args[1].split("/").collect();
                        let filename = file_path[file_path.len()-1];
                        path.push(&filename);

                        let mut file = match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&path){
                              Err(err) =>{
                                    eprintln!("error opening file for send command {}",err);
                                    return Err(err);
                              } ,
                              Ok(file) => (file)
                        };
                        //.unwrap();

                  
                        

                        
                        //request file size from client
                        match write!(&stream, "{}", &"request file size\n"){
                              Ok(_) => {
                                    println!("requested file size");
                                    ()
                              },
                              Err(err) => {
                                    println!("Unable to send command to client in send command during requesting file size: {}", err);
                                    return Err(err);
                              }
                        }

                        let mut reader = BufReader::new(&stream);
                        let mut file_size_buff = Vec::new();

                        match reader.read_until(b'\n', &mut file_size_buff) {
                            Ok(_) => (),
                            Err(err) => {
                                println!("Unable to read into buffer in send command: {}", err);
                            }
                        }
                        file_size_buff.pop();

                        //define file size
                        let file_size = str::from_utf8(&file_size_buff).unwrap().parse::<usize>().unwrap();

                        //request file bytes
                        match write!(&stream, "{}", &"request file\n"){
                              Ok(_) => {
                                    println!("requested file");
                                    ()
                              },
                              Err(err) => {
                                    println!("Unable to send command to server during requesting file to send: {}", err);
                                    return Err(err);
                              }
                        }

                        let mut file_bytes = vec![0u8; file_size];
                        match reader.read_exact(&mut file_bytes) {
                              Ok(_) => (),
                              Err(err) => {
                                  println!("Unable to read into buffer in send command: {}", err);
                              }
                          }

                        //write to file
                        match file.write(&file_bytes){
                              Ok(_) => (),
                              Err(err) => {
                                  println!("Could not write file bytes to file in send command: {}", err);
                              }
                        };

                        println!("Write successful\n");

                  },

                  "receive" if args.len() > 1 => {
                        let filename = &args[1];

                        let mut path = PathBuf::new();
                        path.push("./users_server");
                        path.push(&username);
                        path.push(&filename);

                        //verify if file exists
                        let contents = write_file_to_string(&path);
                        if contents != "Problem opening the file"{
                              match write!(&stream, "{}", &"file found\n") {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to client in receive command: {}", err);
                                    }
                              }

                              let mut reader = BufReader::new(&stream);
                              let mut buffer: Vec<u8> = Vec::new();

                              match reader.read_until(b'\n', &mut buffer) {
                                  Ok(_) => (),
                                  Err(err) => {
                                      println!("Unable to read into buffer in receive command during reading file: {}", err);
                                  }
                              }

                              let buffer = match str::from_utf8(&buffer) {
                                  Ok(buffer) => buffer,
                                  Err(err) => {
                                    //panic!("Unable to read into buffer!");
                                    println!("Could not write buffer as string: {}", err);
                                    continue
                                  
                                  }
                              };
                              
                              if buffer == "request file size\n"{
                                    let mut filebytes: Vec<u8> = Vec::new();

                                    match File::open(&path){
                                        Ok(mut f) => {
                                            match f.read_to_end(&mut filebytes){
                                                Ok(_)=>(),
                                                Err(err)=>{
                                                    println!("Unable to read file to be sent: {}", err);
                                                }
      
                                            };
                                            ()
                                        },
                                        Err(err) => {
                                            println!("Unable to open into file to be sent to client: {}", err);
                                        }
                                    };
      
                                    let mut file_size = filebytes.len().to_string();
                                    file_size.push_str("\n");
      
                                    match stream.write(&file_size.as_bytes()){
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to send file size to client: {}", err);
                                        }
                                    }

                                    let mut buffer: Vec<u8> = Vec::new();
                                    let mut read = BufReader::new(&stream);

                                    match read.read_until(b'\n', &mut buffer) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to read into buffer in receive command: {}", err);
                                        }
                                    }
    
                                    let buf = match str::from_utf8(&buffer) {
                                        Ok(buffer) => buffer,
                                        Err(err) => {
                                            //panic!("Unable to read into buffer!");
                                            println!("Unable to read into buffer in receive command!: {}", err);
                                            continue
                                          
                                        }
                                    };

                                    if buf == "request file\n"{
                                          match stream.write(&filebytes){
                                                Ok(_) => (),
                                                Err(err) => {
                                                    println!("Unable to send bytes to client during requesting file: {}", err);
                                                }
                                          }
                                    }
                                    else{
                                          println!("Error reading client request");
                                    }
                              }
                              else{
                                    println!("Error reading client request");
                              }     


                        }
                        else{
                              match write!(&stream, "{}", &"file not found\n") {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to client during sending file not found: {}", err);
                                    }
                              }
                        }


                        
                  },
                   
                 // "list" if is_authenticated && args.len() > 1 && args[1] == "files" => {
                 "list" if args.len() > 1 && args[1] == "files" => {
                        println!("list files:: user command: {}",cmd_line);

			let mut path = PathBuf::new();
                       path.push("./users_server/");
             		path.push(&username);
                                  		
                                        
                       let r_directory=fs::read_dir(path);
                       let r_directory=match r_directory {
                       	Err(e) =>{
                               	eprintln!("Path problem in 'list files' command: cannot find the user's directory under users_server directory to list files:{:?}", e);
                                       return Err(e)
                                } 
                                Ok(r_directory) => (r_directory)
                       };
                       let mut file_list=String::from("");
                       for entry_res in r_directory.filter_map(Result::ok){
            
                       	let entry = entry_res;
                                            
                      	let this_file_name_buf = entry.file_name();
                      		
                       	let this_file_name = this_file_name_buf.to_str();
                                       
                       	let this_file_name=match this_file_name{
                                None =>{
                                       eprintln!("error in list files the is no such a file under the directory");
                                       break
                                } ,
                                Some(this_file_name)=> this_file_name.to_string()
                        	};
                       	file_list.push_str(this_file_name.as_str()); 
                       	file_list.push('\n');    
			}
			file_list.push('\r'); 
                      	match write!(stream, "{}", &file_list){
                               Ok(_) => (),
                               Err(err) => {
                                      println!("Unable to send command to client in list files: {}", err);
                                      return Err(err);
                                }
                                                         
                        }
                                                            
		   },
                  "login"=>{
                        println!("login attempt");
                        println!("user command: {}",cmd_line);

                      
                        
                       
                        //let mut already_created:i8=0;
                        let mut user_path=String::from("users_server/");

                        user_path.push_str(tokens[1]);

                        if Path::new(&user_path).exists(){
                              //already_created=1;
                             //user exists
                              println!("exist user with this name {}",tokens[1]);


                              match write!(stream, "{}", &"request encrypted\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send request encrypted to client: {}", err);
                                          //return Err(err);
                                    }
                                    }

                              let mut reader = BufReader::new(&stream);
                              let mut msg7=String::from("");
                              match reader.read_line(&mut msg7) {
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to read into buffer: {}", err);
                                          return Err(err);
                                    }


                                             
                              }
                              //msg7 is the oldencryptedtext coming from the client

                              let oldencrypted_text=write_file_to_string_string(&[&user_path, "/txt.encrypt"].join(""));

                              if msg7.trim()== oldencrypted_text {
                                    println!("Old encrypted matched.Requesting new encrypted");

                                    match write!(stream, "{}", &"Old encrypted matched.Requesting new encrypted\n"){
                                          Ok(_) => (),
                                          Err(err) => {
                                                println!("Unable to send command to during requesting new encrypted: {}", err);
                                                //return Err(err);
                                          }
                                          }
      
                                    let mut reader = BufReader::new(&stream);
                                    let mut msg8=String::from("");    //msg8 is the new encrypted from the client
                                    match reader.read_line(&mut msg8) {
                                          Ok(_) => (),
                                          Err(err) => {
                                                println!("Unable to read into buffer: {}", err);
                                                return Err(err);
                                          }
                                    }

                                    let mut user_enc_file = File::create([&mut user_path, "/txt.encrypt"].join(""))?;
                                    user_enc_file.set_len(0)?;
                                    user_enc_file.write_all(msg8.trim().as_bytes())?;


                                    match write!(stream, "{}", &"Successful authentication\n"){
                                          Ok(_) => (),
                                          Err(err) => {
                                                println!("Unable to send command to client during sending Successful authentication message: {}", err);
                                                //return Err(err);
                                          }
                                          }
      
                                    //is_authenticated=true;
                                    

                                    // access the data by mutably borrowing the guard
                                   //let vec = access(&mut guard);
                                    username=tokens[1].to_string();
                                    println!("Successful authentication");
                                    println!("Current User: {}",username);
                                    let active_file =OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    //.create(true)
                                    .open("active.txt");

                                    let mut vec_usernames=words_from_file("active.txt");
                                    vec_usernames.push(username.to_owned());
                                    
                                    let usernames=vec_usernames.connect(" ");
                                    
                                    //let mut active_file = File::create("active.txt");


						match active_file{
						   Ok(mut active_file)=>{
							     match active_file.set_len(0){
								     Ok(_)=>{
									  match active_file.write_all(usernames.as_bytes()){
										Ok(_) => (),
										Err(err) => {
											println!("Unable to read into buffer during reading active users file: {}", err);
											return Err(err);
										}
									  }
								     },
								     Err(err) => {
									println!("Unable to read into buffer during reading active users file: {}", err);
									return Err(err);
								}
							     } 

							    
						   }
						   Err(err) => {
							println!("error opening encrypted: {}", err);
							return Err(err);
							
						}

						}

                                    //active_users.push(username2);
                                    //drop(active_users);
                                   
                                     


                                    
                              }else{
                                    println!("login failed.Wrong encrypted\n");  
                                    match write!(stream, "{}", &"login failed.Wrong encrypted\n"){
                                          Ok(_) => (),
                                          Err(err) => {
                                                println!("Unable to send command 'login failed.Wrong encrypted' to client: {}", err);
                                                //return Err(err);
                                          }
                                          }
                                    }
      
                                              



                         
                        }else{
                              println!("User is not created. Please try again.");

                              match write!(stream, "{}", &"User is not created. Please try again.\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to server: {}", err);
                                          //return Err(err);
                                    }
                                    }
                        }

                  }, // end login

			"logout" => 
			{

                        let mut reader = BufReader::new(&stream);
                        let mut msg12=String::from("");    //msg8 is the new encrypted from the client
                        match reader.read_line(&mut msg12) {
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to read into buffer in logout command: {}", err);
                                    return Err(err);
                              }
                        }

                       
                        let mut active_users=words_from_file("active.txt");
                        //let active_users=vec_usernames.join(" ");
                       
                        if let Some(pos) = active_users.iter().position(|x| *x == username) {
                              active_users.remove(pos);
                          };
                        

                        let active_file =OpenOptions::new()
                          .read(true)
                          .write(true)
                          .create(true)
                          .open("active.txt");
                        
                        //let mut active_file = File::create("active.txt");

                        match active_file{
                           Ok(mut active_file)=>{
                                   match active_file.set_len(0){
                                         Ok(_)=>{
                                          active_file.write_all(active_users.join(" ").as_bytes())?  
                                         },
                                         Err(err) => {
                                          println!("Unable to read into buffer in show active command: {}", err);
                                          return Err(err);
                                    }
                                   } 

                                  
                           }
                           Err(err) => {
                              println!("error opening encrypted: {}", err);
                              return Err(err);
                              
                        }

                        }









                    
                        stream.shutdown(std::net::Shutdown::Both)?;
                      
                        
                  }, // end logout
                  "show" => 
			{
                        if tokens[1]=="users"{
                              

                              let r_directory=fs::read_dir("./users_server/");
                              let r_directory=match r_directory {
                                    Err(e) =>{
                                      eprintln!("Path problem in 'show users' command: cannot find the users_server directory to registered users {:?}", e);
                                      return Err(e)
                                          } 
                                   Ok(r_directory) => (r_directory)
                                    };
                        let mut users=String::from("");                 
                        for entry in r_directory.filter_map(Result::ok){

                              let this_file_name_buf = entry.file_name();
                              let this_file_name = this_file_name_buf.to_str();
                                            
                              let this_file_name=match this_file_name{
                                    None =>{
                                          eprintln!("error");
                                          break
                                    } ,
                                    Some(this_file_name)=> this_file_name.to_string()
                                    };
                                    users.push_str(" ");
                                    users.push_str(this_file_name.as_str());

                        }
                        println!("{}",users);

                        match write!(stream, "{}{}", &users.trim(),"\n"){
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to send command to client: {}", err);
                                    //return Err(err);
                              }
                              }


                        }else if tokens[1]=="active"{
                         
                              
                              let path=String::from("active.txt");
                              let mut active_one=write_file_to_string_string(&path);
                              if active_one==""{
                                 active_one.push_str("there is no active user for now");
                              }
                              match write!(stream, "{}{}", &active_one,"\n"){
                                    Ok(_) => (),
                                    Err(err) => {
                                          println!("Unable to send command to client: {}", err);
                                          //return Err(err);
                                    }
                                    }

                              //let active_users_str: String = active_users.connect(" ");

                              //drop(active_users);


                        }else{
                              println!("Invalid show command!!!");
                        }

                  }, // end show
               

			_ => 
			{
				println!("catch all for now");
                        	
				//if !authenticated_user && tokens[0] == "search" {
				//	response = "only authenticated users may use search\n";
				//} else {	
				//	let response = "Invalid command line -- Please check syntax :)\n#";
					let response = "Invalid command line -- Please check syntax :)\n\r";
				//}
 				match write!(stream, "{}", &response){
                                       Ok(_) => (),
                                       Err(err) => {
                                              println!("Unable to send command to client: {}", err);
                                              return Err(err);
                                       }
                                                               
                               }

			},	
		} // end match on cmd
		
		// echo data back for now
		//stream.write(&buffer[..bytes_read])?;

            // match stream.write(&buffer[..bytes_read]) {
            //       Ok(_) => (),
            //       Err(err) => return Err(err)
            //   }
			
	} // end loop on commands
		
	
}

fn main() {
      let active_file = File::create("active.txt");
      match active_file{
            Err(err) =>{
                  eprintln!("error creating active.txt {}",err);
                  return;
            } ,
            Ok(active_file) => (active_file)

            };
 	let addrs = [
      		SocketAddr::from(([127, 0, 0, 1], 2000)),
      		SocketAddr::from(([127, 0, 0, 1], 7070)),
      	];
      	let listener = TcpListener::bind(&addrs[..]).expect("Unable to bind");
      	println!("Server listening on port: {}", listener.local_addr().unwrap());
      
      
	//let listener = TcpListener::bind("127.0.0.1:2000") // client to connect to this port : 2000
	//			.expect("Unable to bind"); // return listener or panic
    
     
   ;
	//let mut authenticated_user = false;
	// incoming is iterator on connected streams
	// loop on incoming client connections
	for stream in listener.incoming() {
            //let active_users_clone=active_users_shared.clone();
		match stream {
			Err(e) => {	eprintln!("failed: {}", e) }
			Ok(stream) => {
                        
				thread::spawn(move || { //spawn thread on connection
                              //let mut shared=active_users_clone.lock().unwrap();
                              //let mut shared = active_users_here.lock().unwrap();
                              //let mut lock = c_mutex.try_lock();
					connection_thread(stream)
					.unwrap_or_else(|error| eprintln!("{:?}", error));
				});
			}
		}
	}
}





fn words_from_file(filename: &str) -> Vec<String> {
      let mut file = match File::open(filename) {
          Ok(file) => file,
          Err(_) => panic!("no such file"),
      };
      let mut file_contents = String::new();
      file.read_to_string(&mut file_contents)
          .ok()
          .expect("failed to read!");
      let lines: Vec<String> = file_contents.split(" ")
          .map(|s: &str| s.to_string())
          .collect();
      lines
  }


fn write_file_to_string(path1:&PathBuf)-> String
{
      let file = fs::File::open(&path1);
      let mut contents = String::new();

       let mut file=match file {
          Ok(file) => file,
          Err(error) => {
                    eprintln!("Problem opening the file: {}",error);
                  return "Problem opening the file".to_string();
             
          }
       };

       file.read_to_string(&mut contents).map_err(
            |err| println!("{:?}", err)

                  ).ok();
              
     
      contents.to_string()
}



/*
fn write_file_to_string(path1:&PathBuf)-> String
{
      
      let file =OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(&path1);
      //let file = fs::File::open(&path1);
      let mut contents = String::new();

       let mut file=match file {
          Ok(file) => file,
          Err(error) => {
                    eprintln!("Problem opening the file: {}",error);
                  return "Problem opening the file".to_string();
             
          }
       };

       file.read_to_string(&mut contents).map_err(
            |err| println!("{:?}", err)

                  ).ok();
              
     
      contents.to_string()
}
*/

fn write_file_to_string_string(path1:&String)-> String
{
      
      let file =OpenOptions::new()
      .read(true)
      .write(true)
      // .create(true)
      .open(&path1);
      //let file = fs::File::open(&path1);
      let mut contents = String::new();

       let mut file=match file {
          Ok(file) => file,
          Err(error) => {
                    eprintln!("Problem opening the file: {}",error);
                  return "Problem opening the file".to_string();
             
          }
       };

       file.read_to_string(&mut contents).map_err(
            |err| println!("{:?}", err)

                  ).ok();
              
     
      contents.to_string()
}



trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}




