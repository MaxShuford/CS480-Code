"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("#start").value = localStorage.getItem("start");
    $("#destination").value = localStorage.getItem("destination");
    showAlternateRoutes();
    showImage();

    $(".backButton").addEventListener("click", backToWP);
});

function showAlternateRoutes()
{
    //const routes = localStorage.getItem("routes")
    for (let i = 0; i < routes.length; i++){

        //Create New List Item
        const newLi = document.createElement("li");
        newLi.textContent = routes[i];

        //Create Text of Route X then add as child of li
        const newText = document.createElement("Span");
        newText.textContent = "Route " + i;
        newLi.appendChild(newText);

        //Create new button, give event then add as child of li
        const newButton = document.createElement("button");
        newButton.textContent = "View Directions";
        newButton.addEventListener("click", event => {
            localStorage.setItem("routes", routes[i]);
        });
        newLi.appendChild(newButton);

        //add Li as child of ul
        $("aside ul").appendChild(newLi);
    }
}

function showImage()
{
    const postData = {routes: localStorage.getItem("routes")};
    fetch('/mapWithRoutes', {
    method: 'POST', // Specify the method
    headers: {
        'Content-Type': 'application/json', // Inform the server the body is JSON
    },
    body: JSON.stringify(postData), // Convert the JavaScript object to a JSON string
    })
    .then(response => response.json())
    .then(data => {
    console.log('Success:', data);
    const image = JSON.parse(data).image
    $("img").src = "data:image/png;base64," + image;
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

//Back to home button PROTOTYPE
const backToWP = () => {

    //What data should be brought back to Waypoints?


    location.href = "Waypoints.html";
}
