$(document).ready(function() {
    $("#transaction").submit(function(e) {
        e.preventDefault();
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())),
            sell: $("#sell:checked").val() === "on" ? true : false,
        }      

        data = JSON.stringify(data);

        $.ajax({
            url: 'http://127.0.0.1:8080/',
            type: 'post',
            data: data,
            headers: {"Content-Type": "application/json"},
            dataType: 'json',
            success: function (data) {
                console.info(data);
            }
        });
    })
})

function connect() {
    var url = "ws://127.0.0.1:8080/ws/"

    conn = new WebSocket(url);
    console.log("Connecting...");

    conn.onopen = function() {
        console.log("Connected");

        setTimeout(function() {
            conn.send(JSON.stringify({size: 34, price: 3, sell: false}))
        }, 1000)
    }
    conn.onmessage = function(e) {
        console.info("Received a message ", e.data);
    }
    conn.onclose = function() {
        console.warn("Disconnected from server")
        conn = null;
    }
}