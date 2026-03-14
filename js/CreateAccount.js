"USE STRICT";

const $ = selector => document.querySelector(selector);

const backToLogin = () => {

    location.href = "Login.html";
}

document.addEventListener("DOMContentLoaded", () => {
    $("[type=button]").addEventListener("click", event => {
        event.preventDefault();
        const username = $("[name=username]");
        const password = $("[name=password]");
        const confirmPassword = $("[name=confirmPassword]");
        let validUsername = validateUsername(username);
        let validPassword = validatePassword(password, 0);
        let validConfirmPassword = passwordsMatch(password, confirmPassword);
        if(validUsername && validPassword && validConfirmPassword)
        {
            createAccount(username.value, password.value);
        }
    });

    $(".backButton").addEventListener("click", backToLogin);
});

