"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("[type=button]").addEventListener("click", event => {
        event.preventDefault();
        const oldPassword = $("[name=oldPassword]");
        const newPassword = $("[name=newPassword]");
        const confirmPassword = $("[name=confirmPassword]");
        let validOldPassword = validatePassword(oldPassword);  
        let validNewPassword = validatePassword(newPassword);
        let validConfirmPassword = passwordsMatch(newPassword, confirmPassword);
        let verifyPassword = verifyPassword(oldPassword);
        if(validOldPassword && validNewPassword && validConfirmPassword && verifyPassword)
        {
            changePassword(newPassword.value);
        }
    });
});

