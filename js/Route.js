"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("#start").value = localStorage.getItem("start");
    $("#destination").value = localStorage.getItem("destination");
    showAlternateRoutes();
    showImage();
});

function showAlternateRoutes()
{
    const routes = localStorage.getItem("routes")
    for (let i = 0; i < routes.length; i++){
        const newLi = document.createElement("li");
        newLi.textContent = routes[i];
        $("aside ul").appendChild(newLi);
        const newButton = document.createElement("button");
        newButton.textContent = "View Directions";
        $("aside ul:last-child").appendChild(newButton);
        $("aside ul:last-child button").addEventListener("click", event => {
            localStorage.setItem("routes", routes[i]);
        });
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
