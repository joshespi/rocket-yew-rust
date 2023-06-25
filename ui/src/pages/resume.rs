use yew::prelude::*;

pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Resume {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <header>
            <h1>{"Josh Espinoza"}</h1>
            <p>{"Full Stack Development Operations Engineer"}</p>
          </header>

          <main>
            <aside>
              <img src="/data/images/espi.jpg" alt="Josh Espinoza" />
              <h2>{"About Me"}</h2>
              <p>
               {" A skilled and enthusiastic DevOps Engineer with a passion for
                problem-solving and a strong command of Docker, PHP, HTML/CSS. Always
                eager to learn new technologies and expand my skillset. With a solid
                background in front-end and back-end development, I apply a creative
                mindset and a drive for innovation to deliver high-quality solutions."}
              </p>
              <p>
               {" My ability to collaborate effectively with cross-functional teams
                ensures smooth project execution and customer satisfaction. Looking
                for opportunities to leverage my expertise and continue growing in a
                dynamic and challenging environment."}
              </p>

              <h2>{"Contact"}</h2>
              <ul>
                <li><strong>{"Phone: "}</strong>{"(801) 836-0157"}</li>
                <li><strong>{"Location: "}</strong>{"Spanish Fork, Utah"}</li>
                <li>
                  <strong>{"LinkedIn: "}</strong>
                  <a href="https://www.linkedin.com/in/josh-espinoza-200ab7210/"
                    >{"https://www.linkedin.com/in/josh-espinoza-200ab7210/"}</a
                  >
                </li>
                <li>
                  <strong>{"Email: "}</strong>
                  <a href="contact@iamjoshespinoza.com"
                    >{"contact@iamjoshespinoza.com"}</a
                  >
                </li>
                <li>
                  <strong>{"Github: "}</strong>
                  <a href="https://github.com/joshespi"
                    >{"https://github.com/joshespi"}</a
                  >
                </li>
              </ul>

              <h2>{"Hobbies"}</h2>
              <ul>
                <li>{"Web 3.0"}</li>
                <li>
                  {"Learning new things (most recently the world of finance and
                  investing)"}
                </li>
                <li>{"Building PCs"}</li>
                <li>{"Spending time with family"}</li>
              </ul>
            </aside>
            <section>
              <h2>{"Work Experience"}</h2>
              <article>
                <h3>{"DevOps Engineer"}</h3>
                <p>{"| Provo City School District | July 2022 - Present"}</p>
                <ul>
                  <li>
                   {" Designed and executed CI/CD pipelines for various software
                    projects, accelerating development cycles and ensuring quality and
                    reliability of software releases."}
                  </li>
                  <li>
                    {"Leveraged Docker and other containerization technologies to deploy
                    and manage applications across multiple environments, optimizing
                    resource utilization and performance."}
                  </li>
                  <li>
                    {"Collaborated as part of security teams to adhere to industry
                    standards and best practices, enforcing secure coding practices
                    and performing regular vulnerability scans and audits."}
                  </li>
                  <li>
                    {"Implemented robust backup and disaster recovery strategies,
                    safeguarding critical data and systems from potential failures and
                    data loss."}
                  </li>
                  <li>
                    {"Contributed to agile software development methodologies, using
                    Github and other tools to facilitate effective communication and
                    collaboration across teams."}
                  </li>
                  <li>
                   {"Resolved production issues quickly and efficiently, providing
                    technical support and troubleshooting expertise to ensure optimal
                    system availability and performance."}
                  </li>
                  <li>
                    {"Researched and evaluated new tools and technologies to enhance
                    development and deployment processes, staying abreast of industry
                    trends and best practices."}
                  </li>
                  <li>
                    {"Effectively led and managed a dynamic development team to
                    successfully deliver high-quality projects within specified
                    timelines, ensuring seamless coordination and collaboration among
                    team members. Demonstrated exceptional leadership skills in
                    delegating tasks, providing guidance, and fostering a productive
                    work environment, resulting in the successful completion of
                    multiple projects"}
                  </li>
                </ul>
              </article>
              <article>
                <h3>{"Web Development Engineer"}</h3>
                <p>{"| Provo City School District | July 2011 - July 2022"}</p>
                <ul>
                  <li>
                    {"Built responsive and user-friendly web applications using
                    cutting-edge front-end technologies such as HTML, CSS, and
                    JavaScript."}
                  </li>
                  <li>
                    {"Developed efficient and optimized server-side code using powerful
                    back-end technologies like PHP, Python."}
                  </li>
                  <li>
                    {"Worked with UI/UX designers to create visually appealing and
                    intuitive user interfaces."}
                  </li>
                  <li>
                   {" Integrated third-party APIs and services (e.g., payment gateways,
                    social media platforms, or mapping services) into web applications
                    to enhance functionality and user experience."}
                  </li>
                  <li>
                   {" Worked with cross-functional teams (designers, developers, and
                    stakeholders) to gather requirements and deliver projects on time
                    and within budget."}
                  </li>
                  <li>
                   {" Stayed abreast of the latest web development trends, best
                    practices, and emerging technologies, incorporating them into
                    projects when appropriate."}
                  </li>
                  <li>
                    {"Resolved bugs and addressed performance issues through
                    troubleshooting, debugging, and refactoring code."}
                  </li>
                  <li>
                    {"Actively participated in agile development methodologies to ensure
                    effective communication and collaboration across teams."}
                  </li>
                </ul>
              </article>
              <article>
                <h3>{"Advanced Level Skills:"}</h3>
                <ul class="advanced">
                  <li>{"PHP"}</li>
                  <li>{"Python"}</li>
                  <li>{"Docker"}</li>
                  <li>{"Ubuntu/Linux"}</li>
                  <li>{"Servers"}</li>
                  <li>{"HTML/CSS"}</li>
                  <li>{"Javascript"}</li>
                  <li>{"Wordpress"}</li>
                  <li>{"MySQL/MariaDB"}</li>
                </ul>
              </article>
            </section>
          </main>
            </div>
        }
    }
}
