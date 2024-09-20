#let body_font_size = 10.5pt
#let header_font_size = 13pt
#let title_font_size = 16pt
#let email = "amit.rahman@u.nus.edu"
#let website = "amitrahman.me"
#let github = "github.com/amitrahman1026"
#let linkedin = "linkedin.com/in/amitrahman1026"
#let phone_number = "+1 (551) 998-3381"

#set text(font: "Arial", hyphenate: false, size: body_font_size)
#set page(
  margin: (x: 1.1cm, y: 1.3cm),
)

#let chiline() = {v(-2pt); line(length: 100%, stroke: rgb("#777777")); v(-5pt)}

#let section(body, header: "", hide: false) = {
  if hide [] 
  else [
    #pad(top: 3pt, bottom: -5pt)[
    == #upper[#text(size: header_font_size)[#header]]
    ]
    #chiline()
    
    // #v(-3pt)
    #body
    #v(-3pt)
  ]
}

#let experience(
  body, 
  company: "", 
  role: "", 
  start: "", 
  end: "", 
  hide: false,
  location: "Singapore",
  url: ""
) = {
  if hide []
  else {
    if start == "Incoming" [
      #link(url)[*#company*] #h(1fr) #start\
      #role #h(1fr) #location\ 
      #body
    ] else [
      #link(url)[*#company*] #h(1fr) #start -- #end\
      #role #h(1fr) #location\ 
      #body
    ]
  }
}

#let education(
  body,
  school: "",
  degree: "",
  graduation: datetime.today(),
  gpa: ""
) = [
  #let date = { 
    if graduation > datetime.today() [ 
      Expected: #graduation.display("[month repr:short] [year]")
    ] else [ 
      #graduation.display("[month repr:short] [year]") 
    ] 
  }
  
  *#school* #h(1fr) #date \
  #degree
  #body
]

#link("https://" + website)[#text(title_font_size)[= Amit #underline[Rahman]]]


#link("mailto:" + email)[#email] #text(black)[$space.hair$|$space.hair$] #link("https://" + github)[#github] #text(black)[$space.hair$|$space.hair$] #link("https://" + linkedin)[#linkedin] #text(black)[$space.hair$|$space.hair$] #phone_number

#section(header: "Work Authorization")[
  Singapore Citizen; eligible for H-1B1 and J-1 US visas, no petition & lottery required
]

#section(header: "Education")[
  #education(
    school: "National University of Singapore (NUS)",
    degree: "Bachelor of Engineering (Honours) in Computer Engineering",
    graduation: datetime(year: 2025, month: 5, day: 1)
  )[
    - *NUS Engineering Scholar* - Awarded to 50 engineering students in faculty cohort of 1500
  ]
]

#section(header: "Experience")[
  #experience(
    company: "Grasshopper",
    role: "Software Engineer Intern",
    start: "Sep 2024",
    end: "Present",
    location: "Singapore",
    url: "https://grasshopperasia.com/"
  )[
    - Spearheading new developments of a high-performance Rust implementation parser generator for Google's Protobuf, enhancing data serialization efficiency
    - Collaborating with the trading systems team to optimize for low-latency communication protocols, maintaining stability
    - Conducting thorough performance benchmarking ensuring parser meets stringent high-frequency trading requirements
  ]
  
  #experience(
    company: "Kisi Inc.",
    role: "Software Engineer I",
    start: "Feb 2024",
    end: "Aug 2024",
    location: "New York",
    url: "https://www.getkisi.com/"
  )[
    - Led the transition of 50,000 access control IoT devices to FreeRTOS and a GCP-backed proprietary IoT platform, improving system reliability and scalability
    - Developed critical firmware components including a thread-safe networking library and encryption module, resulting in an 80% reduction in networking latency and a 100x improvement in encryption/decryption speed
    - Collaborated with cross-functional teams to implement embedded telemetry support, enhancing overall system monitoring
  ]

  #experience(
    company: "Coditioning",
    role: "Software Engineer Intern",
    start: "Jul 2023",
    end: "Sep 2023",
    location: "Remote",
    url: "https://beta.coditioning.com"
  )[
    - Engineered an end-to-end sandboxed code execution environment with a responsive feedback system, achieving an average response time of 500ms for \~1,000 users
    - Designed and implemented REST API and message queue system, ensuring efficient handling of code submissions.
    - Developed intuitive database schemas and API endpoints for retrieving historical user submissions, enhancing the platform's learning analytics capabilities
  ]

  #experience(
    company: "HydraX",
    role: "Software Engineer Intern",
    start: "May 2023",
    end: "Aug 2023",
    location: "Singapore",
    url: "https://www.hydrax.io/"
  )[
    - Implemented internal cost analysis tools in Kubernetes, improving granularity from cluster to container level and enabling targeted optimization strategies
    - Identified and resolved inefficiencies in log ingestion monitoring stack, reducing cloud storage costs across various clusters by up to 40%
    - Migrated legacy AWS resources to infrastructure-as-code, expanding automation coverage of CI/CD pipeline and improving deployment consistency for private digital exchanges
  ]

  #experience(
    company: "K3 Ventures",
    role: "VC Summer Analyst",
    start: "Jan 2021",
    end: "Mar 2021",
    location: "Singapore",
    url: "https://www.k3ventures.com/"
  )[
    - Developed Python programs to scrape and analyze IPO stock price trends, providing valuable insights for VC clients
    - Created custom monitoring dashboards using data visualization techniques, enhancing the firm's ability to track and evaluate potential investment opportunities
    - Conducted market research on emerging tech trends, contributing to firm's investment strategy in high-growth sectors
  ]
]

#section(header: "Skills")[
  *Languages:* C/C++, Rust, Python, TypeScript, JavaScript, SQL, HTML, CSS, Bash \
  *Technologies:* Docker, Kubernetes, AWS, GCP, GitHub Actions, FreeRTOS, CMake, Linux, Git, Grafana \
  *Frameworks:* React.js, Next.js, Node.js, Express.js, PostgreSQL, Redis, RocksDB
]

// #section(header: "Projects")[
//   - *RESP2 Protocol Compatible Key-Value Database Server Library (Rust):* Developed a high-performance, asynchronous database server library using Rust's m:n runtime model. Implemented in-memory data structures with durable write-ahead logs for external consistency.
//   - *6502 Microprocessor Emulator C++ Library:* Created a cycle-accurate emulator for the 6502 microprocessor, implementing its full instruction set and behavior. Designed a modular architecture using C++ and integrated with the Boost library.
//   - *Other Projects:* Developed various projects in Rust (Port Scanning tool), Kotlin (Android App), Java (Timetable Manager CLI), and maintained a technical blog.
// ]
