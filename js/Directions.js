"USE STRICT";

const $ = selector => document.querySelector(selector);

document.addEventListener("DOMContentLoaded", () => {
    $("button").addEventListener("click", event => {
        favorite();
    });
    showImage();
    showDirections();
});

function showImage()
{
    //const image = localStorage.getItem("image");
    //$(img).src = image;
}

function showDirections()
{
    
    //const directions = localStorage.getItem("directions")
    const directions = ["go that way","go this way", "turn around 30 feet ago", "go back 20 feet", "go forward 10 feet", "turn left", "turn right", "go up the stairs", "go down the stairs"];

    for (let i = 0; i < directions.length; i++){
        const newLi = document.createElement("li");
        newLi.textContent = directions[i];
        $("aside ul").appendChild(newLi);
    }
}

function favorite()
{
    /*
    const postData = { name: 'New User', job: 'Developer' };

    fetch('/', {
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
    */
}