


const bgm = new Audio('assets/music.mp3');
bgm.loop = true;
bgm.volume = 0.5;

let isStarted = sessionStorage.getItem('bgm_started') === 'true';

if (isStarted) {
    const savedTime = sessionStorage.getItem('bgm_time');
    if (savedTime) {
        bgm.currentTime = parseFloat(savedTime);
    }
    bgm.play().catch(err => console.warn("Autoplay blocked on navigation:", err));
}

export function initBGM() {
    if (isStarted) return; // Already started

    const handleInteraction = () => {
        bgm.play()
            .then(() => {
                isStarted = true;
                sessionStorage.setItem('bgm_started', 'true');
                document.removeEventListener('click', handleInteraction);
                document.removeEventListener('keydown', handleInteraction);
            })
            .catch((err) => {
                console.warn("Autoplay still blocked, waiting for next interaction:", err);
            });
    };

    document.addEventListener('click', handleInteraction);
    document.addEventListener('keydown', handleInteraction);
}

window.addEventListener('beforeunload', () => {
    if (isStarted && !bgm.paused) {
        sessionStorage.setItem('bgm_time', bgm.currentTime.toString());
    }
});

export function togglePlay() {
    if (bgm.paused) {
        bgm.play().catch(console.error);
        sessionStorage.setItem('bgm_started', 'true');
    } else {
        bgm.pause();
        sessionStorage.setItem('bgm_started', 'false');
    }
}

export function stopBGM() {
    bgm.pause();
    bgm.currentTime = 0;
    sessionStorage.setItem('bgm_started', 'false');
}

export function setVolume(val) {
    bgm.volume = Math.max(0, Math.min(1, val));
}

initBGM();