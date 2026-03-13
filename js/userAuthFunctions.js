"USE STRICT";

/*
    Validates that the username meets the username requirements
    param: username - the input element for the username
    return: true if the username is valid, false if not
*/
function validateUsername(username)
{
    if(username.value === ""){
        username.nextElementSibling.textContent = "Username is required.";
        username.nextElementSibling.classList.remove("hidden");
        return false;
    }
    else if(username.value.length < 3 || username.value.length > 30)
    {
        username.nextElementSibling.textContent = "Username must be between 3 and 30 characters.";
        username.nextElementSibling.classList.remove("hidden");
        return false;
    }
    else
    {
        username.nextElementSibling.textContent = "";
        return true;
    }
}

/*
    Validates that the password meets the password requirements
    param: password - the input element for the password
              code - 0 for change password, 1 for login (used to determine error message)
    return: true if the password is valid, false if not
*/
function validatePassword(password, code)
{
    let passrequirmentsmet = 0;
    if(password.value === ""){
        password.nextElementSibling.textContent = "Password is required.";
        password.nextElementSibling.classList.remove("hidden");
        return false;
    } 
    else if(password.value.length < 8 || password.value.length > 25)
    {
        password.nextElementSibling.textContent = "Password must be between 8 and 25 characters.";
        password.nextElementSibling.classList.remove("hidden");
        return false;
    }
    else{
        let regex = /[A-Z]/;
        if(regex.test(password.value))
            passrequirmentsmet++;
        regex = /[a-z]/;
        if(regex.test(password.value))
            passrequirmentsmet++;
        regex = /[0-9]/;
        if(regex.test(password.value))
            passrequirmentsmet++;
        regex = /[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/;
        if(regex.test(password.value))
            passrequirmentsmet++;

        if(passrequirmentsmet < 3)
        {
            if(code == 0){
                password.nextElementSibling.textContent = "Password must contain at least 3 of the following: uppercase letters, lowercase letters, numbers, and special characters.";
                password.nextElementSibling.classList.remove("hidden");
            }
            else{
                password.nextElementSibling.textContent = "Invalid Password";
                password.nextElementSibling.classList.remove("hidden")
             }
            return false;
        }
        else
        {
            password.nextElementSibling.textContent = "";
            return true;
        }
    }
}

/*
    Validates that the confirm password matches the new password
    param: password - the input element for the new password
              confirmPassword - the input element for the confirm password
    return: true if the passwords match, false if not
*/
function passwordsMatch(password, confirmPassword)
{
    if(confirmPassword.value === "")
    {
        confirmPassword.nextElementSibling.textContent = "Please confirm your password.";
        return false;
    }
    if(password.value !== confirmPassword.value)
    {
        confirmPassword.nextElementSibling.textContent = "Passwords do not match.";
        return false;
    }
    else
    {
        confirmPassword.nextElementSibling.textContent = "";
        return true;
    }
}

/*
    Logs the user in by sending a POST request to the server with the username and hashed password
    param: username - the username of the user
        password - the password of the user
*/
function login(username, password) {
    //hash the password
    hashString(password).then(hashedPassword => {
        password = hashedPassword;
    });
    
    //create and send the POST request to the server
    const postData = { username: username, hashed_pw: password};
    let userID;
    fetch('/login', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    userID = userID;
    localStorage.setItem('username', username);
    localStorage.setItem('userID', userID);
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

/*
    Changes the user's password by sending a POST request to the server with the userID, old hashed password, and new hashed password
    param: user - the userID of the user
        oldPassword - the old password of the user
        newPassword - the new password of the user
*/
function changePassword(user, oldPassword, newPassword)
{
    hashString(oldPassword).then(hashedPassword => {
        oldPassword = hashedPassword;
    });

    hashString(newPassword).then(hashedPassword => {
        newPassword = hashedPassword;
    });
    
    const postData = { userID: user, old_pw: oldPassword, new_pw: newPassword };

    fetch('/changePassword', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

/*
    Creates an account by sending a POST request to the server with the username and hashed password
    param: username - the username of the user
        password - the password of the user
*/
function createAccount(username, password) 
{
    hashString(password).then(hashedPassword => {
        password = hashedPassword;
    });

    const postData = { username: username, hashed_pw: password };

    fetch('/createAccount', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

/*
    Hashes a string using the SHA-256 algorithm and returns the hash as a hex string
    param: message - the string to be hashed
    return: the hashed string as a hex string
*/
async function hashString(message) {
    // Encode the string as a Uint8Array
    const msgBuffer = new TextEncoder().encode(message); 
    
    // Hash the message using SHA-256
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer); 
    
    // Convert the ArrayBuffer to a hex string
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(b => ('00' + b.toString(16)).slice(-2)).join('');
    console.log(hashHex);
    return hashHex;
}

/*
    logs the user out of the signed in account
*/
function logout()
{
    localStorage.setItem("username", "");
    localStorage.setItem("userID", "");
}