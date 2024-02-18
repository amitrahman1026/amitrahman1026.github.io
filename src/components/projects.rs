use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="projects-container">
            <h1>{ "Projects" }</h1>

            <div class="project">
                <a href="https://github.com/JothamWong/Procastinot" target="_blank">{"Screen Time Limit Android App"}</a>
                <p>{"Kotlin | Jetpack Compose"}</p>
                <p>{"An innovative Android application designed to help users manage their screen time effectively using Kotlin and Jetpack Compose. Features include OS-agnostic CLI-based university timetable & academic planner with persistent local storage."}</p>
            </div>

            <div class="project">
                <a href="https://github.com/amitrahman1026/my6502" target="_blank">{"6502 Microprocessor Emulator C++ Library"}</a>
                <p>{"C/C++ | ASM | CMake | Boost"}</p>
                <p>{"A C++ library emulating the 6502 microprocessor, facilitating accurate simulation for enthusiasts and developers. The project leverages CMake for project management and Boost for unit testing, ensuring emulator reliability and correctness."}</p>
            </div>

            <div class="project">
                <a href="https://github.com/amitrahman1026/EE2026-Final-Project" target="_blank">{"Morse Code Flashcard on FPGA Board"}</a>
                <p>{"FPGA | Verilog HDL | Digital Logic"}</p>
                <p>{"A Morse code flashcard game implemented on an FPGA board, aimed at enhancing learning through spaced repetition. The project integrates GUI, I/O operations, audio spectrogram visualization, and utilizes LSFR for pseudo-randomization."}</p>
            </div>

            <div class="project">
                <a href="https://github.com/amitrahman1026/search-rescue-bot" target="_blank">{"Search and Rescue Robot Vehicle"}</a>
                <p>{"C/C++ | ROS | Linux | Arduino | UART | TLS"}</p>
                <p>{"A tele-operated robot designed for search and rescue operations, featuring terrain mapping and collision avoidance. Utilizes LIDAR, ultrasonic sensors, and ROS for communication, with data transmission over UART and secured by TLS."}</p>
            </div>
        </div>
    }
}

