"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("button").addEventListener("click", event => {
        favorite();
    });
    showImage();
    showDirections();

    $(".backButton").addEventListener("click", backToRoutes);
});

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

function showDirections()
{
    const directions = localStorage.getItem("routes");
    for (let i = 0; i < directions.length; i++){
        const newLi = document.createElement("li");
        newLi.textContent = directions[i];
        $("aside ul").appendChild(newLi);
    }
}

function favorite()
{
    const postData = { uid: localStorage.getItem('userID'), route: localStorage.getItem('route') };

    fetch('/addToFavorite', {
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

//Back to route button PROTOTYPE
const backToRoutes = () => {

    //What data should be brought back to


    location.href = "Route.html";
}