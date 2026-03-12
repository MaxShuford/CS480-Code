"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("[value=Login]").addEventListener("click", event => {
        event.preventDefault();
        const username = $("[name=username]");
        const password = $("[name=password]");
        let validUsername = validateUsername(username);
        let validPassword = validatePassword(password, 1);
        if(validUsername && validPassword)
        {
            login(username.value, password.value);
        }
    });
});



