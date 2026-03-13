"USE STRICT";

const $ = selector => document.querySelector(selector);
let routes;
document.addEventListener("DOMContentLoaded", () => {
    $("#start").textContent = localStorage.getItem("start");
    $("#destination").textContent = localStorage.getItem("destination");
    routes = JSON.parse(localStorage.getItem("routes"));
    showAlternateRoutes();
    showImage();
    $(".backButton").addEventListener("click", backToWP);
});

function showAlternateRoutes()
{
    for (let i = 0; i < routes.length; i++){

        //Create New List Item
        const newLi = document.createElement("li");
        //newLi.textContent = routes[i];

        //Create Text of Route X then add as child of li
        const newText = document.createElement("Span");
        newText.textContent = "Route " + (i+1);
        newLi.appendChild(newText);

        const link = document.createElement("a");
        link.href = "/html/Directions.html";
        newLi.appendChild(link);

        //Create new button, give event then add as child of li
        const newButton = document.createElement("button");
        newButton.textContent = "View Directions";
        newButton.classList.add("buttons");
        newButton.addEventListener("click", event => {
            localStorage.setItem("routes", JSON.stringify(routes[i]));
        });
        link.appendChild(newButton);

        //add Li as child of ul
        $("aside ul").appendChild(newLi);
    }
}

function showImage()
{
    console.log(routes);
    const postData = [];
    for(let i = 0; i < routes.length;i++)
    {
        postData[i]= {route:{route_id:i, wp:routes[i].waypoints}, geometry:routes[i].geometry};
    }
    console.log("post", postData);
    console.log(JSON.stringify(postData))
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
    $("#theMap").src = "data:image/png;base64," + data.image;
    })
    .catch((error) => {
    console.error('Error:', error);
    });
}

