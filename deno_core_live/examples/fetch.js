function print(data) {
    Deno.core.print(`${data}\n`); 
}


print("starting to fetch..."); 
let res =  await fetch({ url: "https://www.baidu.com", method: 'POST' }); 
print(`status: ${res.status}`);
print(`headers: ${JSON.stringify(res.headers)}`);
print(`body: ${res.text()}`);