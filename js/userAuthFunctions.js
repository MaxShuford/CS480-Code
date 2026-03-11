"USE STRICT";


function validateUsername(username)
{
    if(username.value === ""){
        username.nextElementSibling.textContent = "Username is required.";
        return false;
    }
    else if(username.value.length < 3 || username.value.length > 30)
    {
        username.nextElementSibling.textContent = "Username must be between 3 and 30 characters.";
        return false;
    }
    else
    {
        username.nextElementSibling.textContent = "";
        return true;
    }
}

function validatePassword(password)
{
    let passrequirmentsmet = 0;
    if(password.value === ""){
        password.nextElementSibling.textContent = "Password is required.";
        return false;
    } 
    else if(password.value.length < 8 || password.value.length > 25)
    {
        password.nextElementSibling.textContent = "Password must be between 8 and 25 characters.";
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
            password.nextElementSibling.textContent = "Password must contain at least 3 of the following: uppercase letters, lowercase letters, numbers, and special characters.";
            return false;
        }
        else
        {
            password.nextElementSibling.textContent = "";
            return true;
        }
    }
}

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

function verifyPassword(oldPassword)
{
    /*
    const postData = { name: 'New User', job: 'Developer' };

    fetch('/', {
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
    */

    if(oldPassword.value === localStorage.getItem("password"))
    {
        return true;
    }
    else
    {
        oldPassword.nextElementSibling.textContent = "Incorrect old password.";
        return false;
    }
}

function login(username, password) {

    /*
    const postData = { name: 'New User', job: 'Developer' };

    fetch('/', {
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
    */

    hashString(password).then(hashedPassword => {
        localStorage.setItem('password', hashedPassword);
    });

    localStorage.setItem('username', username);
    
}

function changePassword(newPassword)
{
    /*
    const postData = { name: 'New User', job: 'Developer' };

    fetch('/', {
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
    */

    hashString(password).then(hashedPassword => {
        localStorage.setItem('password', hashedPassword);
    });
}

function createAccount(username, password) 
{
    /*
    const postData = { name: 'New User', job: 'Developer' };

    fetch('/', {
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
    */

    localStorage.setItem('username', username);
    hashString(password).then(hashedPassword => {
        localStorage.setItem('password', hashedPassword);
    });
}

async function hashString(message) {
    // Encode the string as a Uint8Array
    const msgBuffer = new TextEncoder().encode(message); 
    
    // Hash the message using SHA-256
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer); 
    
    // Convert the ArrayBuffer to a hex string
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(b => ('00' + b.toString(16)).slice(-2)).join('');
    
    return hashHex;
}
