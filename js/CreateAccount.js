"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("[type=button]").addEventListener("click", event => {
        event.preventDefault();
        const username = $("[name=username]");
        const password = $("[name=password]");
        const confirmPassword = $("[name=confirmPassword]");
        let validUsername = validateUsername(username);
        let validPassword = validatePassword(password);
        let validConfirmPassword = passwordsMatch(password, confirmPassword);
        if(validUsername && validPassword && validConfirmPassword)
        {
            createAccount(username.value, password.value);
            login(username.value, password.value);
        }
    });
});

