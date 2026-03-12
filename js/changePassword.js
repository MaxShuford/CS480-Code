"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("[type=button]").addEventListener("click", event => {
        event.preventDefault();
        const oldPassword = $("[name=oldPassword]");
        const newPassword = $("[name=newPassword]");
        const confirmPassword = $("[name=confirmPassword]");
        let validOldPassword = validatePassword(oldPassword, 0);  
        let validNewPassword = validatePassword(newPassword, 0);
        let validConfirmPassword = passwordsMatch(newPassword, confirmPassword);
        if(validOldPassword && validNewPassword && validConfirmPassword)
        {
            changePassword(localStorage.getItem("userID"), oldPassword.value, newPassword.value);
            $("#changePassword").submit();
        }
    });

    $(".backButton").addEventListener("click", backToWP);
});

//Back to home button PROTOTYPE
const backToWP = () => {

    //What data should be brought back to Waypoints?


    location.href = "Waypoints.html";
}

