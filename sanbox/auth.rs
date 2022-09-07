/*   println!("the server response is {:#?}", &zeus_server_response);
    let res = zeus_server_response.unwrap().text().await;
    println!("{:?}", res); */
    //connect to database, check if user details is valid
    let database = config::database::mongodb().await;
    let collection = database.collection::<User>("user_information");

    let result = collection
        .find_one(doc! { "username": &username }, None)
        .await
        .unwrap();

    //try to destructure the found object, then ask for password
    if let Some(User {
        username,
        password: hashed_password,
        ..
    }) = result
    {
        let password: String = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password: ")
            .interact()
            .unwrap();

        //check for correctness of the pasword
        let is_correct_password = verify(&password, &hashed_password);
        match is_correct_password {
            Ok(correct_password) => {
                //destruct password
                if !correct_password {
                    println!("{} {}", style("Incorrect Password for ").red(), &username);
                    return;
                }
            }
            //inform the user of the error
            Err(_) => {
                println!("{} {}", style("Error authorizing",).red(), &username);
                return;
            }
        }

        //in the password is correct, begin the chat
        println!(
            "Successfully logged in as {}\nType \".help\" for more information.\n",
            &username,
        );

        //the help information
        let help_information = r#"
.clear    Clear the current input
.editor   Enter editor mode
.exit     Exit the REPL
.help     Print this help message
        "#;

        //chat guide
        let chat_guide = r#"
\b           begin chat
\i <email>   invite a friend via chat
\e           end conversation
\j <id>      join a chat via id
        "#;

        'outer: loop {
            let mut repl = rustyline::Editor::<()>::new().unwrap();
            let readline = repl.readline(">> ");
            //check the user input
            match readline {
                Ok(input) => {
                    if input.trim() == ".help" {
                        println!("{}", &help_information);
                    } else if input.trim() == ".break" {
                        break 'outer;
                    } else if input.trim() == ".editor" {
                        println!("Entered editor mode\n{}", style(chat_guide).cyan());
                        'inner: loop {
                            let mut repl = rustyline::Editor::<()>::new().unwrap();
                            let prompt = style(">> ").cyan().to_string();
                            let readline = repl.readline(&prompt);
                            match readline {
                                Ok(message) => println!("you entered {}", message),
                                Err(_) => {
                                    println!("an error occurred");
                                    break 'inner;
                                }
                            }
                        }
                    } else if input.trim() == ".exit" {
                        break;
                    } else {
                        println!(
                            "{}\nType \".help\" for more information",
                            style("Invalid input.").red()
                        );
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("An unexpected error occurred{:?}", err);
                    break;
                }
            }
        }
    } else {
        //if no user was found return 404 error
        println!(
            "{}",
            style("User with provided credentials not found").red()
        );
    };