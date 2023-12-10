document.addEventListener('DOMContentLoaded', () => {
    const messagesDiv = document.getElementById('messages');
    const newMessageForm = document.getElementById('new-message');

    // Function to append a message to the 'messages' div
    function appendMessage(username, message) {
        const messageElement = document.createElement('div');
        messageElement.textContent = `${username}: ${message}`;
        messagesDiv.appendChild(messageElement);
    }

    // Handle form submission
    newMessageForm.addEventListener('submit', (e) => {
        e.preventDefault();

        const username = newMessageForm.username.value.trim();
        const message = newMessageForm.message.value.trim();

        if (username && message) {
            // Send the message to the server
            fetch('/message', {
                method: 'POST',
                body: new URLSearchParams({ 'room': 'general','username': username, 'message': message }),
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            });

            newMessageForm.message.value = ''; // Clear the message input field
        }
    });

    // Set up the SSE connection
    const eventSource = new EventSource('/events');

    eventSource.onmessage = (event) => {
        const data = JSON.parse(event.data);
        appendMessage(data.username, data.message);
    };
});
