"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    //$("#start").value = localStorage.getItem("start");
    //$("#destination").value = localStorage.getItem("destination");
    $("#start").textContent = "Ellensburg";
    $("#destination").textContent = "Seattle";
    showAlternateRoutes();
    showImage();
});

function showAlternateRoutes()
{
    //const routes = localStorage.getItem("routes")
    const routes = ["i-90", "W-12", "P-50"];
    for (let i = 0; i < routes.length; i++){
        const newLi = document.createElement("li");
        newLi.textContent = routes[i];
        $("aside ul").appendChild(newLi);
        const newButton = document.createElement("button");
        newButton.textContent = "View Directions";
        $("aside ul:last-child").appendChild(newButton);
        $("aside ul:last-child button").addEventListener("click", event => {
            getDirectiosns(i);
        });
    }
}

function showImage()
{

}

function getDirectiosns(routeNum)
{
    
}