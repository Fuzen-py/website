(function () {
    const MIN_LENGTH = 4;
    const API_BASE = "https://fuzen.info"
    const API_URL = `${API_BASE}/randomword?min=${MIN_LENGTH}`
    const wordLine = document.querySelector('#word_line');
    const remainingSelector = document.querySelector('#remaining');
    const highScoreSelector = document.querySelector("#high_score");
    const guesses = document.querySelector('#guesses');
    const validKeys = 'abcdefghijklmnopqrstuvwxyz'.split('');
    let high_score = 0;
    // Blanks is a string
    let word;
    let blanks;
    let max_attempts;
    let attempts = [];

    function updateDisplay() {
        wordLine.innerText = blanks.join(' ');
        remainingSelector.innerText = (max_attempts - attempts.length);
        guesses.innerText = (attempts.length != 0) ? attempts.join(', ') : '';
    }
    // Displays a loss screen
    function lossScreen() {
        wordLine.innerText = `You lose, it was "${word}"`;
        remainingSelector.innerText = '';
        guesses.innerText = '';
    }

    // Displays a win screen
    function winScreen() {
        score = (max_attempts - attempts.length) / (word.length + 2) * 100 >> 0
        wordLine.innerText = `"${word}" You win!`;
        remainingSelector.innerText = `Score: ${score}`;
        if (score <= highScore) {
            return
        }
        highScoreSelector.innerText = score;
        highScore = score;
    }

    // Reset
    function reset() {
        document.body.removeEventListener('keypress', onKeyPress)
        word = undefined;
        blanks = undefined;
        max_attempts = 0;
        attempts = [];
        getWord(); // Call getWord again
    }
    function onKeyPress(e) {
        if (validKeys.includes(e.key.toLowerCase())) {
            let letter = e.key.toLowerCase();
            if (attempts.length === max_attempts || blanks.join('') === word) {
                return;
            }
            // if letter is in word
            if (word.includes(letter)) {
                // Edit blanks
                for (var i = 0; i < word.length; i++) {
                    if (letter === word[i]) {
                        blanks[i] = letter;
                    }
                }
                // Compare if blanks is the same as word
                if (blanks.join('') === word) {
                    return winScreen();
                }
                // update display if not
                return updateDisplay();
            }

            // If letter is not in word but in attempts
            if (attempts.includes(letter)) {
                return;
            }
            // Letter is not in word and not in attempts
            attempts.push(letter);
            if (attempts.length === max_attempts) {
                return lossScreen();
            }
            return updateDisplay();
        }
        // Reset
        if ('`' === e.key) {
            reset();
        }
    }

    function init(resp) {
        // Store the response
        word = resp.responseText.toLowerCase();
        blanks = Array(word.length).fill('_');
        max_attempts = word.length + 2;
        document.body.addEventListener('keypress', onKeyPress);
        updateDisplay();
    }
    // Gets a new word then passes it into init
    function getWord() {
        let req = new XMLHttpRequest();
        wordLine.innerText = "Please wait...";
        remainingSelector.innerText = "";
        req.open("GET", API_URL, true)
        req.onreadystatechange = function () {
            if (req.readyState === XMLHttpRequest.DONE && req.status === 200) {
                init(req)
            }
        };
        req.send();
    }
    getWord()

}())