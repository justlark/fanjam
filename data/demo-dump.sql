--
-- PostgreSQL database dump
--

-- Dumped from database version 17.5 (84bec44)
-- Dumped by pg_dump version 17.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: _nc_m2m_people_events; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx._nc_m2m_people_events (events_id, people_id) FROM stdin;
\.


--
-- Data for Name: _nc_m2m_tags_events; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx._nc_m2m_tags_events (events_id, tags_id) FROM stdin;
10	3
11	1
12	1
19	1
23	1
33	1
35	3
36	1
40	3
1	4
14	4
32	4
5	5
7	5
11	5
21	5
22	5
25	5
26	5
27	5
29	5
30	5
31	5
39	5
41	5
18	6
13	6
12	6
9	7
8	7
35	7
4	8
8	8
6	8
10	8
40	8
33	8
\.


--
-- Data for Name: about; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.about (id, created_at, updated_at, created_by, updated_by, nc_order, con_name, con_description, website, files) FROM stdin;
1	2025-08-23 10:44:52	2025-09-19 03:13:12	usi0a68hopccoh8h	usju2x29g7c6afe9	1	GeekCon	A con by and for geeks of all kinds!	https://example.org	[{"id":"atl0nru96gxf07lv","url":"https://sparklefish-noco-bugbot.151bc8670b862fa7d694cf7246a2c0dc.r2.cloudflarestorage.com/nc/uploads/noco/pg60bi600000bjx/mu79cb3zl9zm62o/c0827nscw5iqezj/Venue Map__Jsq0.jpg","title":"Venue Map","mimetype":"image/jpeg","size":472614,"width":2500,"height":1408}]
\.


--
-- Data for Name: announcements; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.announcements (id, created_at, updated_at, created_by, updated_by, nc_order, title, description, attachment) FROM stdin;
2	2025-08-23 11:12:38	2025-08-23 11:12:48	usi0a68hopccoh8h	usi0a68hopccoh8h	2	Dealers' Den is now open!	\N	\N
1	2025-08-23 11:12:26	2025-09-18 10:36:49	usi0a68hopccoh8h	usju2x29g7c6afe9	1	Elevator Maintenance	\N	\N
\.


--
-- Data for Name: categories; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.categories (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
1	2025-08-23 10:39:38	2025-09-16 10:42:57	usi0a68hopccoh8h	usju2x29g7c6afe9	1	Meetup
4	2025-08-23 10:41:53	2025-09-16 10:49:15	usi0a68hopccoh8h	usju2x29g7c6afe9	4	Activity/Workshop
3	2025-08-23 10:40:33	2025-09-16 10:50:26	usi0a68hopccoh8h	usju2x29g7c6afe9	3	Panel
2	2025-08-23 10:39:42	2025-08-23 12:29:35	usi0a68hopccoh8h	usju2x29g7c6afe9	2	Competition
\.


--
-- Data for Name: events; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.events (id, created_at, updated_at, created_by, updated_by, nc_order, name, description, start_time, end_time, locations_id, categories_id, hidden, summary) FROM stdin;
3	2025-08-22 14:29:50	2025-09-21 12:32:08	usju2x29g7c6afe9	usbmwbrxagzg3seb	3	Cosplay Competition	Enter the battle for ultimate cosplay supremacy!<br><br>Come dressed as a favorite character from video games, movies, tv, books, or any other fandom you want to embody! <br>Our panel of expert judges will decide who reigns supreme with winners in the categories of best dressed overall, best genderbending cosplay, best OC, best non-human character, and best youth cosplay. \n\n <br> As we want this to be an inclusive experience for our young congoers please no nudity or sexual content in contest entries. 	2025-09-21 18:00:00	2025-09-21 20:00:00	3	2	f	Embody your favorite character and show off your skills in a battle for cosplay supremacy!
11	2025-08-23 10:55:39	2025-09-21 12:26:48	usi0a68hopccoh8h	usbmwbrxagzg3seb	11	Queer Otome Games Showcase	While traditional otome games are written for a female main character choosing between male love interests, the genre has been expanding in recent years with broader gender and sexuality representation for both main character and love interests, aro/ace representation, trans representation, and more. This panel will feature some of the highlights from recent years as well as new offerings from our panelists to satisfy your romantic visual novel desires.	2025-09-20 18:00:00	2025-09-20 19:30:00	4	3	f	Panelists will show off their favorite queer otome games as well as their own projects
2	2025-08-22 14:29:09	2025-09-21 12:29:03	usju2x29g7c6afe9	usbmwbrxagzg3seb	2	Fursuit Dance Competition	Bring your best moves to center stage with a very furry dance-off! We'll have prizes and competitions galore so come wag your tail, clap your paws, and show us what you've got. Full suiting is strongly encouraged, but all creatures are welcome. We'll have water and snacks on hand to keep you going all night long.	2025-09-21 00:00:00	2025-09-21 01:00:00	3	2	f	Bring your best moves to center stage with a very furry dance-off! 
23	2025-08-23 12:26:43	2025-09-21 12:50:39	usju2x29g7c6afe9	usbmwbrxagzg3seb	23	Night Market	Our vendors hall opens up for 18+ shopping time after dark, and vendors will put all their spiciest NSFW goods on display for you to peruse. We'll also have a commisions corner where you can connect with artists who are ready to illustrate your fantasies, and a stamp rally to guide you through the darker side of the marketplace and help you find some new and exciting things to take home.	2025-09-21 00:00:00	2025-09-21 02:00:00	8	4	f	A late-night shopping extravaganza for anyone in search of something strange and unusual. 
8	2025-08-22 14:40:19	2025-09-21 12:57:46	usju2x29g7c6afe9	usbmwbrxagzg3seb	8	Fanfic Writers Workshop	Join a fanfic writer-turned published author for a hands-on workshop to develop your skills as a writer. They'll share tips on setting a scene, pacing and storyboarding, writing dialogue, and more. Practical exercises will help you hone your skills, and then we'll have a fishbowl-style critique to help dive deep into the skills discussed in the workshop.<br>Bring a short piece of writing (500 words or less) that you'd like to improve. Participation in the fishbowl is not required for attending this workshop, but peer critique is one of the best and fastest ways to improve!	2025-09-20 16:00:00	2025-09-20 17:30:00	1	4	f	Develop your writing skills for better fanfiction and share your writing with your peers.
5	2025-08-22 14:34:20	2025-09-21 12:55:56	usju2x29g7c6afe9	usbmwbrxagzg3seb	5	Queer Gamers Meetup	Meet up with your fellow queer gamers for a relaxed and unstructured hangout. Talk about representation in games, find a new squad, show off your high scores, and just generally enjoy each others' company.<br>This meetup is open to queer-identified fans of all gaming platforms--video games, board games, TTRPGs, etc. Allies, we love gaming with you, but please reserve this space for queer-identified folks.	2025-09-21 19:00:00	2025-09-21 20:00:00	1	1	f	A meetup for queer-identified folks of all gaming styles
13	2025-08-23 11:05:36	2025-09-21 12:34:32	usi0a68hopccoh8h	usbmwbrxagzg3seb	13	The Music of Fullmetal Alchemist: Brotherhood	As a lover of classical music and anime, I'm always ready to talk about how musical themes, motifs, instrumentation, tempo, and rhythm add to the emotion and narrative of a show. We'll listen to selected songs from the FMA: Brotherhood OST with context from the show and I'll talk about how the music supports the series with a little music history education sprinkled in.	2025-09-20 21:30:00	2025-09-20 23:00:00	4	3	f	Listen to FMA: Brotherhood's music together with a classical music enthusiast
20	2025-08-23 12:16:39	2025-09-21 12:56:27	usju2x29g7c6afe9	usbmwbrxagzg3seb	20	Tickets, Please: Railfan Meetup	All aboard! If you love trains and rail travel, this is the meetup for you. Railfans will have a chance to meet one another, talk about their favorite trains and routes, share videos, and get to know each other.	2025-09-21 20:00:00	2025-09-21 21:00:00	1	1	f	Meetup for fans of trains and rail travel
15	2025-08-23 11:14:42	2025-09-21 13:00:23	usju2x29g7c6afe9	usbmwbrxagzg3seb	15	Furry Meetup	Meow? Woof! Awooooooo!!! Come meet fellow creatures of all persuasions at the furry meetup. Furries, therians, petplayers, and friends are all welcome at this meetup. Suiting is encouraged, but not required. 	2025-09-20 21:30:00	2025-09-20 22:30:00	1	1	f	Meet fellow furries at a casual and comfortable meetup.
33	2025-09-16 10:33:57	2025-09-21 12:59:06	usju2x29g7c6afe9	usbmwbrxagzg3seb	32	NSFW Sketchbook Swap	A sketchbook swap for 18+ art afficionados. Bring your sketchbook and create some NSFW art for your fellow artists and get some in return. This is a sex-positive, body-positive art space where all forms of self-expression is welcome, but please ask for consent and offer content warnings for sexually explicit or violent content. 	2025-09-20 20:00:00	2025-09-20 21:00:00	1	4	f	NSFW sketchbook swap meetup
39	2025-09-16 10:45:58	2025-09-21 13:13:49	usju2x29g7c6afe9	usbmwbrxagzg3seb	38	Ergonomics for PC Gamers	Think PC gaming isn't a physical sport? Think again! Come learn about the physiology of gaming, from posture considerations like seat and desk setup to repetitive stress injury prevention strategies. This comprehensive talk will cover a number of topics to keep you gaming and healthy well into your old age. We'll discuss seating, equipment positioning, ergonomic equipment like mouse and keyboard, posture, supportive tools like wrist braces and lumbar pillows, behavioral strategies like stretching and break schedules, and more. We hope you'll leave with the skills and knowledge to prevent repetitive stress injury, back pain, eye strain, and generally protect your health while playing your favorite games.	2025-09-20 13:00:00	2025-09-20 14:30:00	4	3	f	Learn how to protect your health without sacrificing your gaming fun.
41	2025-09-16 10:49:53	2025-09-21 13:36:07	usju2x29g7c6afe9	usbmwbrxagzg3seb	40	Adaptive Tech For Accessible Gaming	Gaming with a disability can be an extremely frustrating experience. For many people, gaming has felt completely inaccessible because the interface is made for one type of person with very little flexibility to accommodate individual differences. But fans have long taken matters into their own hands to make the world of gaming open to everyone. Come learn about adaptive controllers, creative sensory feedback, customized gaming rigs, and more ways people have found to make games possible for themselves. We'll bring a few of our favorite adaptive controllers for you to try out after our talk!	2025-09-20 20:00:00	2025-09-20 21:30:00	2	3	f	Try out some of our favorite adaptive tech and learn about grassroots initiatives to make gaming more accessible.
34	2025-09-16 10:37:47	2025-09-21 12:33:27	usju2x29g7c6afe9	usbmwbrxagzg3seb	33	PowerPoint Karaoke	It's karaoke! It's Powerpoint! No, it's Powerpoint Karaoke! You'll be given a random slide deck from a collection amassed over years of presentation parties and con talks and figure out how to present it as best you can on the fly. Presenters will have 5 minutes to talk about their topic followed by 2 minutes of Q&A. Who knows what you'll have to pretend to be an expert on? 	2025-09-21 15:00:00	2025-09-21 16:30:00	3	4	f	Pretend to be an expert and present on a random topic using a random slide deck (provided by our host)
40	2025-09-16 10:46:14	2025-09-21 13:28:18	usju2x29g7c6afe9	usbmwbrxagzg3seb	39	Fiber Arts Crafternoon	Come craft along with us! We'll be teaching a beginner-friendly crochet project for you to make a soft pet dragon to take home. Class fee is $30 and you'll receive all materials you need to start the project and finish it at home. Please RSVP to give us a rough idea of how much yarn to bring. Feel free to bring your own fiber arts project and craft along with the group if you'd like to hang out but don't need another project. 	2025-09-21 16:00:00	2025-09-21 19:00:00	1	4	f	Come craft with us! Either bring your own fiber arts project or do a craft-along crochet dragon project to take home!
32	2025-09-16 10:28:15	2025-09-21 12:24:17	usju2x29g7c6afe9	usbmwbrxagzg3seb	31	Beyond D&D: TTRPG Systems You've Never Heard Of	Have you ever played a TTRPG where the primary mechanic is a tumbling block tower? What about a completely silent text-based RPG? Dungeons and Dragons is a classic of the TTRPG genre, and has given so much to the art form, but not everyone wants to do math and pore over spellbooks in their free time (or maybe you're just looking for a change of pace). We've collected some of our favorite non-traditional TTRPGS, with inventive settings, mechanics, themes, and storylines. We'll bring materials from some of our favorites and talk about why we love them so much, and after the talk give you the opportunity to take a look and ask us any questions you've got.	2025-09-20 15:00:00	2025-09-20 16:30:00	4	3	f	Showcasing small, niche, and indie TTRPG systems far from the d20 and the dungeon.
28	2025-09-16 10:15:03	2025-09-21 13:04:58	usju2x29g7c6afe9	usbmwbrxagzg3seb	27	Mechanical Keyboard Swap Meet	Bring your mechanical keyboard and show off the tactical feedback you worked so hard to perfect. Bring old cap sets, switches, and cases that are looking for a new home, and maybe go home with a new project. If you're curious about mechanical keyboards, feel free to show up and ask questions too!	2025-09-21 21:00:00	2025-09-21 10:00:00	1	1	f	Share tips, swap equipment, and show off your keeb!
36	2025-09-16 10:39:45	2025-09-21 13:03:52	usju2x29g7c6afe9	usbmwbrxagzg3seb	35	TNG Munch	The Next Generation munch is exclusively for young adults in the kink community to forge new connections, strengthen existing friendships, and enjoy a community space just for them. 	2025-09-20 23:00:00	2025-09-21 01:00:00	1	1	f	Meet other 18-35-year-olds involved in the kink scene
35	2025-09-16 10:38:55	2025-09-21 12:37:03	usju2x29g7c6afe9	usbmwbrxagzg3seb	34	Doctor Who Escape Room	A fun and playful escape room experience themed around our favorite Time Lord. Rely on your intellect and logic, knowledge of Gallifreyan trivia, and your trusty sonic screwdriver to make your way through a series of puzzles and physical challenges to escape in time to save the universe. 	2025-09-21 15:00:00	2025-09-21 16:30:00	5	4	f	An escape room themed around our favorite time lord
29	2025-09-16 10:21:18	2025-09-21 12:48:23	usju2x29g7c6afe9	usbmwbrxagzg3seb	28	Moderating Communities for Guild Masters	Tensions can run high in online guilds, especially when they're filled with hundreds of relatively anonymous players passionate about a game. Group dynamics are complicated at the best of times, but for guild masters who want their group to be cooperative, kind, supportive, and cohesive it can be extremely difficult to keep everyone in line and on each others' side. <br>Learn some strategies for mediating conflict, setting explicit and implicit group norms, planning guild events, creating a cohesive group identity, and more. Bring your dilemmas and wins to the table and connect with others who've struggled and succeeded in the quest to bring people together. 	2025-09-21 15:30:00	2025-09-21 17:00:00	2	3	f	Talk about moderation and mediation in the context of MMORPG guilds. 
38	2025-09-16 10:44:17	2025-09-21 12:44:44	usju2x29g7c6afe9	usbmwbrxagzg3seb	37	Watch Your Step: Dealing With Missing Stairs in Community	Knowing how to navigate community safety is vitally important for protecting yourself from risk and for being a good member of any social space you inhabit. Geek fandoms try to be welcoming, inclusive spaces to the people who have felt like misfits and outcasts, and many of us are used to feeling misunderstood or excluded. But with that commitment to inclusion and acceptance can come a risk of overlooking or making excuses for patterns of harmful action by individual community members. Nobody wants to be the one to "cancel" or call out people making a genuine mistake, and it's so hard to speak up when we see something wrong. But in every community there will be people who, either intentionally or accidentally, consistently hurt other members. This talk will take a serious look at how to respond when someone comes forward to report that they've been harmed, how to keep ourselves and each other safe, how to balance acceptance and empathy with proactive responsibility, and even what to do if you realize _you've_ been the missing stair. **CW**: This will be a heavy talk and will reference subjects like sexual assault, violence, and coercion. 	2025-09-21 20:30:00	2025-09-21 22:00:00	2	3	f	How to respond when realizing a member of your community has repeatedly harmed others.
27	2025-09-16 10:14:40	2025-09-21 12:46:15	usju2x29g7c6afe9	usbmwbrxagzg3seb	26	Fan Games, Copyright, and You	You love your favorite titles, and want to pay homage to the characters and worlds in them. Or maybe you just think you can do it better. Either way, there's a whole world of copyright law out there with traps and pitfalls to avoid when it comes to things like licensing, fair use, and intellectual property. This panel, presented by an actual real-life lawyer, will give you a primer in how to show your love through indie game design _without_ getting fined. They'll show you examples of what to do, and what not to do, and will wrap up with a Q&A where you can ask your burning questions without judgment or legal consequences. <br><br>Please note: while the panelist is speaking from their experience as a lawyer, participation in this panel and the information presented therein does not constitute legal advice in any way. The organizers of this con are not liable for any consequences of you making your own fan game, no matter how absolutely rad it may be.	2025-09-20 13:30:00	2025-09-20 15:00:00	2	3	f	A primer on staying out of trouble when creating your homage to popular titles.
31	2025-09-16 10:26:32	2025-09-21 12:34:50	usju2x29g7c6afe9	usbmwbrxagzg3seb	30	What Makes a Video Game Soundtrack	Music is a powerful emotional force for video game immersion. There are countless examples now of indie developers and big studios releasing games with incredible soundtracks. But what makes a good game soundtrack? How do composers and developers decide where and how to use what songs, what little details to include, how to set the tone and tempo of the music, instrumentation, themes, rhythm, etc. What is the process of designing a soundtrack for a game like, and how is it different from other music composition? How can music add to the experience of a game without distracting the player or throwing them off? This tuneful talk will feature lots of examples demonstrating the principles of video game soundtrack design. 	2025-09-20 20:00:00	2025-09-20 21:30:00	4	3	f	Panel talk about video game soundtracks and sound design.
30	2025-09-16 10:23:05	2025-09-21 12:44:21	usju2x29g7c6afe9	usbmwbrxagzg3seb	29	The Politics of EVE Online: A History	Have you ever heard of the ruthless Goonswarm alliance? What about the terrible doomsday weapon known as Steve?\n\nEVE Online, one of the first space sim MMOs, has a is known for its player-driven economy, warring alliances, and massive battles. Alliances in EVE Online are real people forming real factions outside the game, and the consequences of that in-game conflict sometimes leak out into the real world.\n\nThe politics of EVE Online are complicated and messy, and have been for its more than 20-year history. We're going to dive deep into the lore and history of the major alliances, wars, and battles the game is famous for.	2025-09-21 18:30:00	2025-09-21 20:00:00	2	3	f	Learn about the messy history of politics, intrigue, and interpersonal conflict that the space sim MMO EVE Online is famous for.
21	2025-08-23 12:24:27	2025-09-21 12:40:33	usju2x29g7c6afe9	usbmwbrxagzg3seb	21	Name That Video Game Sound Effect	Think you know your blips from your pings?You'll get the chance to put that knowledge to the test as a panelist on our game show. We'll have a series of progressively harder challenges all focused around the unsung heroes of video game sound design. If you get the most answers write you could walk away with our custom sound-board! But be warned... it might be harder than you think to... Name That Sound Effect!	2025-09-20 15:00:00	2025-09-20 16:30:00	3	2	f	Test out your skill at identifying sound effects from popular (and not-so-popular) video games!
22	2025-08-23 12:25:32	2025-09-21 12:41:22	usju2x29g7c6afe9	usbmwbrxagzg3seb	22	Speedrunning Charity Event	Join together with your fellow gamers to earn money for a good cause. We'll have a speedrunning demonstration and some friendly competition all with the goal of raising funds for the Trevor Project. Come to hang out, watch some games, and try to beat fellow competitors' high scores. Our vendors have generously supplied us with prizes and we're excited to see how much we can rack up to serve our community.	2025-09-20 19:00:00	2025-09-20 21:30:00	5	2	f	A casual semicompetitive speedrunning event to earn money for the Trevor Project
14	2025-08-23 11:07:27	2025-09-21 12:28:08	usi0a68hopccoh8h	usbmwbrxagzg3seb	14	Worldbuilding for GMs	Join our panelists as they demonstrate their favorite world-building techniques. They'll discuss the who, what, when, where and why of an in-game world and give examples of each in action. Whether you want to create a complex landscape, rich economy, or unique magic system, design unique characters with compelling motivations, or create an emotional tone to your game, you'll learn was to draw from fiction, history, and everyday life to make your fantasy world come to life for your players.	2025-09-21 15:00:00	2025-09-21 16:30:00	4	3	f	Panelists talk about strategies for making their game's world feel more real.
16	2025-08-23 11:15:18	2025-09-21 12:29:50	usju2x29g7c6afe9	usbmwbrxagzg3seb	16	Variety Talent Show	Got a hidden talent you've been dying to show off? Get up on stage and give it all you've got! Musical instruments, dance, impressions, showing off a skill or hobby, infodumping about your favorite topic, talents of all kinds are welcome here. Performances will be limited to a max of 5 minutes to ensure all our contestants have time to take part. Make sure to put your name on the sign-up sheet at registration to make sure you get a chance to take part.	2025-09-20 20:00:00	2025-09-20 22:00:00	3	4	f	Show off your hidden talents on stage!
12	2025-08-23 10:57:41	2025-09-21 12:22:50	usi0a68hopccoh8h	usbmwbrxagzg3seb	12	Analyzing Hentai Tropes	If you've ever interacted with hentai, you might find yourself with a few questions. Does anyone really want Z-cup boobs? Why do people keep getting nosebleeds? And what's with all those tentacles? This panel will take a playful look at this unique facet of Japanese culture, how Japanese censorship laws and isolationist policies have played a role in shaping hentai. <br><br>Content warning: This panel is 18+ only and will feature explicit sexual content. Additional content warnings will be provided before individual examples.	2025-09-21 00:00:00	2025-09-21 01:30:00	4	3	f	A playful look at Japanese culture and history through the lens of hentai tropes
18	2025-08-23 11:59:03	2025-09-21 12:45:46	usju2x29g7c6afe9	usbmwbrxagzg3seb	18	Neon Genesis Evangelion Watch Party	One of the most influential and iconic anime series about mechs to ever come out of Japan, _Neon Genesis Evangelion_ shows Shinji Ikari, a 14-year-old boy, piloting a giant robot to fight the massive and terrifying angels in the hopes of earning the approval of his estranged father. We'll watch the first three episodes of this classic masterpiece with snacks, drinks, and a heavy dose of surrealist existentialism. 	2025-09-21 01:00:00	2025-09-21 04:00:00	2	1	f	Movie viewing for the anime series Neon Genesis Evangelion
17	2025-08-23 11:57:27	2025-09-21 12:27:43	usju2x29g7c6afe9	usbmwbrxagzg3seb	17	"Aye, and my boffer": Gear for LARPing	Whether you're new to LARPing or you've been doing it for years, there's always more to learn about the gear. Crafting historically inspired costumes and accessories is a fun way to add depth to your character, and getting the right equipment can make a huge difference in how much fun you have in your next LARP. We'll talk weapons and armor, our favorite shops, and crafting techniques for getting your costume just right. A short show-and-tell after our panel will let you get up close and personal with some of our favorite gear.	2025-09-21 13:00:00	2025-09-21 14:30:00	4	3	f	A panel on the different types of gear used in live action role-playing games. 
19	2025-08-23 12:13:58	2025-09-21 12:48:49	usju2x29g7c6afe9	usbmwbrxagzg3seb	19	Fantasy Roleplay in Kink	Role-playing in kink contexts is not all that different from embodying a character around a D&D table or at a LARP. We have characters and their motivations, dialogue and action, and the magic happens when those characters meet and interact. This panel reviews some of the basics of character design and role-play, with a focus on how to incorporate these principles into kink scenes.	2025-09-20 22:00:00	2025-09-20 23:30:00	2	3	f	Panelists talk about how to adapt roleplaying skills to kink settings
26	2025-09-16 10:13:36	2025-09-21 12:47:40	usju2x29g7c6afe9	usbmwbrxagzg3seb	25	Games Accessibility and "Cheating"	There is a whole world of games accessibility concepts beyond the basics of adaptive controllers and subtitles. Lack of cognitive or neurological accessibility prevents people from enjoying games they'd otherwise love. And while the challenge of a well-designed game is rewarding and satisfying, feeling completely stuck or hopeless is no fun for anyone. Slow or inconsistent reaction time, attention, processing delays, memory deficits, visual or auditory processing differences, etc. should never hold you back from feeling like a real gamer. Supports and accommodations in single-player or casual/collaborative games should be seen as the tool for equity they are rather than "cheating." \n\n<br>In this panel learn about the creative solutions players have found to make games accessible and enjoyable for themselves, from fan-made mods to in-game assists to external tools like game guides, helping with everything from memory and information processing to decision-making to motor control and reaction time and everything in between! See examples of games accessibility done right from across the industry and think about how you might benefit from \n\n<br>Come ready to challenge everything you know about what it means to "cheat" at games, and leave with a little more compassion and flexibility to let yourself just enjoy games without punishing yourself for your limitations.	2025-09-20 18:30:00	2025-09-20 20:00:00	2	3	f	Rethink everything you know about "cheating" at video games with this talk about accessibility and who deserves to enjoy games.
6	2025-08-22 14:35:38	2025-09-21 12:58:51	usju2x29g7c6afe9	usbmwbrxagzg3seb	6	Sketchbook Swap	Blank pages burning a hole in your sketchbook? Come to the sketchbook swap and share your art with others. Trade sketchbooks and draw a doodle, a sketch, a full-page full-color masterpiece, whatever you want! Welcome to all skill levels, art styles, and subjects. 	2025-09-20 19:00:00	2025-09-20 20:00:00	1	4	f	Fill up your sketchbook and share your art at the sketchbook swap
7	2025-08-22 14:35:54	2025-09-21 12:35:16	usju2x29g7c6afe9	usbmwbrxagzg3seb	7	FNAF Lore Deep Dive	_Five Nights at Freddy's_ may have started as a simple game, but the lore has become rich and complex as dozens of games, several books, a movie, and countless other pieces of media have been released over the years. Fans initially drawn to the spooky atmosphere and creepy animatronics have found a canon rich in complex themes like grief, loss of innocence, identity, shame, helplessness, and trauma. <br>This will be a deep dive into the lore of the entire FNAF franchise, with comments about controversy, open questions, and the underlying themes represented throughout. Who is William Afton? Why do these animatronics want to kill you? Is the pizza any good? Come see what makes Freddy Fazbear's a magical place for kids and grown-ups alike.	2025-09-21 21:30:00	2025-09-21 22:30:00	4	3	f	FNAF Lore Deep Dive 
10	2025-08-23 10:51:46	2025-09-21 12:57:02	usi0a68hopccoh8h	usbmwbrxagzg3seb	10	Chainmaille 101	A hands-on introductory workshop giving you all the skills and basic equipment you need to get started in the art of chainmaille. We'll discuss sourcing materials, the basics of opening, closing, and joining rings, resources for patterns and project ideas, and safety considerations. Then, we'll get you started on a simple project of your very own to get you diving right into the hobby. This introduction is designed for all skill levels and abilities, and we can offer advice on adaptive weaving strategies if needed. A $20 dollar materials fee will cover a set of pliers and rings for your first project. Please RSVP with the organizers so we can get an approximate headcount for purchasing materials.	2025-09-20 13:00:00	2025-09-20 15:30:00	1	4	f	An introductory lesson to the skills and equipment used in the art of chainmaille
25	2025-09-16 10:13:04	2025-09-21 12:49:25	usju2x29g7c6afe9	usbmwbrxagzg3seb	24	Game Modding and Hacking	Learn how to glitch, exploit, hack, and make mods for your favorite singleplayer games to push gameplay into strange new directions. Finding exploits and glitches in games offers a curious and creative challenge for those who have mastered a game the "normal" way. Using mods can drastically alter the parameters of a game to make it more interesting, challenging, creative, accessible, or better suited to self-expression and enjoyment! And while we don't condone cheating in a multiplayer context, that is using mods or exploits to gain an unfair advantage over others without their consent, mods and hacks can make your favorite multiplayer games more exciting and fit you and your friends perfectly too. We'll share some of our favorite mod marketplaces for popular games, show off some of our favorite designs, and talk about the history of gamers pushing beyond the boundaries of developer's original intent. 	2025-09-20 15:30:00	2025-09-20 17:00:00	2	3	f	Learn how to glitch, exploit, hack, and mod your favorite games!
37	2025-09-16 10:43:29	2025-09-21 12:36:02	usju2x29g7c6afe9	usbmwbrxagzg3seb	36	Charity Dinner	A charity dinner featuring dance, song, and other performance to entertain you while you eat, followed by a silent auction to peruse over dessert. This will be a buffet-style dinner with options available for vegetarian, gluten-free, and dairy-free diners. If you have another dietary restriction please reach out and we'll see what we can do. A handful of tickets will be available for purchase at the event, but if you want to ensure your spot please RSVP in advance!	2025-09-20 23:00:00	2025-09-21 01:00:00	5	4	f	A ticketed dinner featuring performances to raise money for charity
9	2025-08-22 14:43:09	2025-09-21 12:43:59	usju2x29g7c6afe9	usbmwbrxagzg3seb	9	Creating Inclusive Spaces in Fandom	Fandom is a place for people to come together, to express who we are unapologetically and share the things we love. Fan communities are a wonderful space for us to feel welcome, included, and authentically ourselves. So many of us come into fandom feeling like misfits and weirdos, and the joy of finding a place where we truly belong is life-changing. <br><br>Yet so many fandoms struggle to be inclusive to _all_ fans. Many people end up looking in from the outside longing to be a part of a group that still feels beyond reach, and others step into a place that should be inclusive and welcoming and feel more isolated or outcast than ever.\n\n<br>We'll talk about a range of identities that have historically been excluded from fandoms and why, and explore the role of race, gender, age, disability, socioeconomic status, and more in our fan communities and society at large. Then, we'll talk about practical solutions for creating more inclusive community, from minority representation to communication practices to accessibility accommodations and beyond. Finally, we'll open up the room to hear your experiences in fan communities both good and bad.	2025-09-21 13:30:00	2025-09-21 15:00:00	2	3	f	An interactive panel discussing issues of inclusion and accessibility in fan communities
1	2025-08-22 14:28:39	2025-09-21 12:35:31	usju2x29g7c6afe9	usbmwbrxagzg3seb	1	Magic: The Gathering Tournament	We'll be holding a Commander draft for both new and seasoned players. This event will start with a brief intro to Commander for anyone who hasn't played this format before, followed by the draft and a three-round tournament to see who will rise victorious. Never played before? Don't worry, we've got veterans on standby to help you make sense of the cards, and if you'd like a low-pressure place to learn we can create a noncompetitive table to help beginners get their bearings.<br><br>We have a number of packs available to purchase at the event, or you can feel free to bring your own sealed Commander Legends packs. Please let the organizers know in advance if you need to purchase packs so we can get an estimate of how many to bring.	2025-09-21 18:00:00	2025-09-21 21:00:00	4	2	f	A Commander Legends draft for beginners and seasoned veterans alike.
4	2025-08-22 14:30:56	2025-09-21 12:55:01	usju2x29g7c6afe9	usbmwbrxagzg3seb	4	Warhammer 40k Craft Hour	Ashamed that the last few units in your army are still solid grey? Or maybe your entire army is unpainted? Don't worry, this is a safe space. No matter your mini painting skill level, the size of your army, your aesthetic, your play style, we won't judge. We'll only judge you if you play Tau. XD Just kidding--everyone's welcome here. <br><br>While this is mainly intended to be a space for people to assemble and paint, the minifig-curious are welcome to join. If you've never painted minis before, or you don't have an army, this is a great place to learn. We have some extra models for you to try out painting skills on and get a feel for the hobby. <br><br>\\*Note: due to ventilation and equipment concerns, we will not be able to prime or spray-paint figures in this space. Please be sure to prep your minis before bringing them to the crafting hour 	2025-09-21 13:00:00	2025-09-21 15:00:00	1	4	f	A model-building and painting session for Warhammer 40k players. Other minifigs welcome!
\.


--
-- Data for Name: links; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.links (id, created_at, updated_at, created_by, updated_by, nc_order, name, url) FROM stdin;
1	2025-08-23 11:09:02	2025-08-23 11:09:05	usi0a68hopccoh8h	usi0a68hopccoh8h	1	Hotel Bookings	https://example.org
2	2025-08-23 11:10:58	2025-08-23 11:11:07	usi0a68hopccoh8h	usi0a68hopccoh8h	2	Official Discord	https://example.org
\.


--
-- Data for Name: locations; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.locations (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
8	2025-09-19 19:52:52	2025-09-19 19:52:52	usbmwbrxagzg3seb	usbmwbrxagzg3seb	7	Vendors Hall
5	2025-08-23 00:59:18	2025-09-19 19:59:08	usi0a68hopccoh8h	usbmwbrxagzg3seb	5	Longfellow Ballroom
6	2025-08-23 00:59:49	2025-09-19 21:23:02	usi0a68hopccoh8h	usbmwbrxagzg3seb	6	Dickinson Room
2	2025-08-23 00:54:20	2025-09-19 21:24:26	usi0a68hopccoh8h	usbmwbrxagzg3seb	2	Plath Room
1	2025-08-23 00:54:16	2025-09-19 21:25:05	usi0a68hopccoh8h	usbmwbrxagzg3seb	1	Thoreau Room
4	2025-08-23 00:55:07	2025-09-19 21:25:10	usi0a68hopccoh8h	usbmwbrxagzg3seb	4	Alcott Room
3	2025-08-23 00:54:30	2025-09-19 19:44:39	usi0a68hopccoh8h	usbmwbrxagzg3seb	0.5	Emerson Stage
\.


--
-- Data for Name: pages; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.pages (id, created_at, updated_at, created_by, updated_by, nc_order, title, body, files) FROM stdin;
1	2025-08-23 11:11:25	2025-08-23 11:11:26	usi0a68hopccoh8h	usi0a68hopccoh8h	1	FAQ	\N	\N
2	2025-08-23 11:11:30	2025-08-23 11:11:31	usi0a68hopccoh8h	usi0a68hopccoh8h	2	Accessibility	\N	\N
3	2025-08-23 11:11:40	2025-09-19 03:17:04	usi0a68hopccoh8h	usju2x29g7c6afe9	0.5	Rules & Policies		\N
\.


--
-- Data for Name: people; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.people (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
\.


--
-- Data for Name: tags; Type: TABLE DATA; Schema: pg60bi600000bjx; Owner: sparklefish
--

COPY pg60bi600000bjx.tags (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
7	2025-09-19 01:55:25	2025-09-19 02:01:16	usbmwbrxagzg3seb	usbmwbrxagzg3seb	6	Movies, TV, and Books
8	2025-09-19 01:58:39	2025-09-19 02:02:46	usbmwbrxagzg3seb	usbmwbrxagzg3seb	7	Arts & Crafts
1	2025-08-23 10:39:20	2025-09-16 10:39:57	usi0a68hopccoh8h	usju2x29g7c6afe9	1	18+
3	2025-08-23 10:54:20	2025-09-16 10:49:23	usi0a68hopccoh8h	usju2x29g7c6afe9	2	$$$
4	2025-09-19 01:53:41	2025-09-19 01:57:31	usbmwbrxagzg3seb	usbmwbrxagzg3seb	3	Card, Board, & TTRPG Games
5	2025-09-19 01:53:49	2025-09-19 01:58:34	usbmwbrxagzg3seb	usbmwbrxagzg3seb	4	Video Games
6	2025-09-19 01:55:21	2025-09-19 01:59:38	usbmwbrxagzg3seb	usbmwbrxagzg3seb	5	Anime
\.


--
-- Name: about_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.about_id_seq', 1, true);


--
-- Name: announcements_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.announcements_id_seq', 2, true);


--
-- Name: categories_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.categories_id_seq', 4, true);


--
-- Name: events_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.events_id_seq', 41, true);


--
-- Name: links_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.links_id_seq', 2, true);


--
-- Name: locations_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.locations_id_seq', 8, true);


--
-- Name: pages_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.pages_id_seq', 3, true);


--
-- Name: people_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.people_id_seq', 1, false);


--
-- Name: tags_id_seq; Type: SEQUENCE SET; Schema: pg60bi600000bjx; Owner: sparklefish
--

SELECT pg_catalog.setval('pg60bi600000bjx.tags_id_seq', 8, true);


--
-- PostgreSQL database dump complete
--

