let res = await fetch("https://jsonplaceholder.typicode.com/todos/1"); 
let json = await res.json();
console.log(json)