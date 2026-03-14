"use strict";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {

    const changeBtn = document.querySelector("input[type=button]");

    changeBtn.addEventListener("click", event => {

    event.preventDefault();

    const oldPassword = $("[name=oldPassword]").value;
    const newPassword = $("[name=newPassword]").value;
    const confirmPassword = $("[name=confirmPassword]").value;

    if(newPassword !== confirmPassword){
        alert("Passwords do not match");
        return;
    }

    const uuid = localStorage.getItem("userID");

    changePassword(uuid, oldPassword, newPassword);
});


    const backButton = document.querySelector(".backButton");
    if(backButton){
        backButton.addEventListener("click", () => {
            window.location.href = "/html/Waypoints.html";
        });
    }

});