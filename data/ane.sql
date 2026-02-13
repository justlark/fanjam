--
-- PostgreSQL database dump
--

\restrict k5EzhTwczVc7QaTnpIO2jsruN1ZEdB4A5B1ZcrY9KZrGMkZMmNbrC9ci4iETTOg

-- Dumped from database version 17.7 (bdc8956)
-- Dumped by pg_dump version 18.1

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
-- Name: pjb8x7vqtomqrms; Type: SCHEMA; Schema: -; Owner: sparklefish
--

CREATE SCHEMA pjb8x7vqtomqrms;


ALTER SCHEMA pjb8x7vqtomqrms OWNER TO sparklefish;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _nc_m2m_people_events; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms._nc_m2m_people_events (
    events_id integer NOT NULL,
    people_id integer NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms._nc_m2m_people_events OWNER TO sparklefish;

--
-- Name: _nc_m2m_tags_events; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms._nc_m2m_tags_events (
    events_id integer NOT NULL,
    tags_id integer NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms._nc_m2m_tags_events OWNER TO sparklefish;

--
-- Name: about; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.about (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    con_name text NOT NULL,
    con_description text,
    website text,
    files text
);


ALTER TABLE pjb8x7vqtomqrms.about OWNER TO sparklefish;

--
-- Name: about_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.about_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.about_id_seq OWNER TO sparklefish;

--
-- Name: about_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.about_id_seq OWNED BY pjb8x7vqtomqrms.about.id;


--
-- Name: announcements; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.announcements (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    title text NOT NULL,
    description text,
    attachment text
);


ALTER TABLE pjb8x7vqtomqrms.announcements OWNER TO sparklefish;

--
-- Name: announcements_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.announcements_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.announcements_id_seq OWNER TO sparklefish;

--
-- Name: announcements_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.announcements_id_seq OWNED BY pjb8x7vqtomqrms.announcements.id;


--
-- Name: categories; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.categories (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms.categories OWNER TO sparklefish;

--
-- Name: categories_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.categories_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.categories_id_seq OWNER TO sparklefish;

--
-- Name: categories_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.categories_id_seq OWNED BY pjb8x7vqtomqrms.categories.id;


--
-- Name: events; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.events (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL,
    description text,
    start_time timestamp without time zone,
    end_time timestamp without time zone,
    locations_id integer,
    categories_id integer,
    hidden boolean DEFAULT false,
    summary text
);


ALTER TABLE pjb8x7vqtomqrms.events OWNER TO sparklefish;

--
-- Name: events_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.events_id_seq OWNER TO sparklefish;

--
-- Name: events_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.events_id_seq OWNED BY pjb8x7vqtomqrms.events.id;


--
-- Name: links; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.links (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL,
    url text NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms.links OWNER TO sparklefish;

--
-- Name: links_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.links_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.links_id_seq OWNER TO sparklefish;

--
-- Name: links_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.links_id_seq OWNED BY pjb8x7vqtomqrms.links.id;


--
-- Name: locations; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.locations (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms.locations OWNER TO sparklefish;

--
-- Name: locations_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.locations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.locations_id_seq OWNER TO sparklefish;

--
-- Name: locations_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.locations_id_seq OWNED BY pjb8x7vqtomqrms.locations.id;


--
-- Name: pages; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.pages (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    title text NOT NULL,
    body text,
    files text
);


ALTER TABLE pjb8x7vqtomqrms.pages OWNER TO sparklefish;

--
-- Name: pages_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.pages_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.pages_id_seq OWNER TO sparklefish;

--
-- Name: pages_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.pages_id_seq OWNED BY pjb8x7vqtomqrms.pages.id;


--
-- Name: people; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.people (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms.people OWNER TO sparklefish;

--
-- Name: people_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.people_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.people_id_seq OWNER TO sparklefish;

--
-- Name: people_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.people_id_seq OWNED BY pjb8x7vqtomqrms.people.id;


--
-- Name: tags; Type: TABLE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE TABLE pjb8x7vqtomqrms.tags (
    id integer NOT NULL,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    created_by character varying,
    updated_by character varying,
    nc_order numeric,
    name text NOT NULL
);


ALTER TABLE pjb8x7vqtomqrms.tags OWNER TO sparklefish;

--
-- Name: tags_id_seq; Type: SEQUENCE; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE SEQUENCE pjb8x7vqtomqrms.tags_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE pjb8x7vqtomqrms.tags_id_seq OWNER TO sparklefish;

--
-- Name: tags_id_seq; Type: SEQUENCE OWNED BY; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER SEQUENCE pjb8x7vqtomqrms.tags_id_seq OWNED BY pjb8x7vqtomqrms.tags.id;


--
-- Name: about id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.about ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.about_id_seq'::regclass);


--
-- Name: announcements id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.announcements ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.announcements_id_seq'::regclass);


--
-- Name: categories id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.categories ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.categories_id_seq'::regclass);


--
-- Name: events id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.events ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.events_id_seq'::regclass);


--
-- Name: links id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.links ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.links_id_seq'::regclass);


--
-- Name: locations id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.locations ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.locations_id_seq'::regclass);


--
-- Name: pages id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.pages ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.pages_id_seq'::regclass);


--
-- Name: people id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.people ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.people_id_seq'::regclass);


--
-- Name: tags id; Type: DEFAULT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.tags ALTER COLUMN id SET DEFAULT nextval('pjb8x7vqtomqrms.tags_id_seq'::regclass);


--
-- Data for Name: _nc_m2m_people_events; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms._nc_m2m_people_events (events_id, people_id) FROM stdin;
1	1
2	2
3	3
4	4
5	5
6	3
7	4
8	2
9	4
10	6
11	7
12	8
13	9
14	10
14	11
15	12
16	13
16	14
17	15
18	16
19	17
20	18
22	4
23	19
24	20
25	21
25	22
26	23
27	3
28	24
29	25
30	4
31	26
32	27
33	28
34	4
35	1
36	29
37	30
301	31
302	32
38	33
39	34
40	35
41	36
42	37
42	38
43	39
44	40
45	26
46	41
47	1
48	14
49	15
49	42
50	43
50	44
51	45
52	46
53	47
54	48
55	40
56	49
57	26
58	50
59	51
60	52
60	53
61	54
61	55
62	56
63	28
64	57
65	58
66	59
67	60
68	61
69	62
69	63
70	64
71	65
72	26
73	4
74	66
75	67
77	68
78	69
78	70
79	59
80	71
80	72
81	73
82	74
82	75
83	26
84	76
85	77
86	78
87	79
88	80
89	81
90	82
90	83
91	1
92	84
93	74
94	85
95	4
96	86
97	68
98	1
99	4
100	4
101	87
102	88
103	89
104	45
105	80
106	90
106	91
107	34
108	1
109	92
110	93
110	94
110	95
111	96
112	97
113	98
114	99
115	16
116	8
117	100
118	101
119	4
120	102
120	103
121	104
122	105
123	106
123	107
123	108
124	109
125	110
125	111
126	112
127	113
127	114
128	115
128	116
129	117
130	118
130	119
131	120
131	89
131	121
132	106
133	122
133	28
134	80
135	123
136	124
137	125
138	110
139	126
139	127
140	26
141	128
142	1
143	129
143	130
144	131
144	132
145	133
146	134
147	135
148	4
149	136
149	137
150	138
150	139
151	140
152	141
152	65
153	26
154	142
155	143
155	144
156	145
157	146
158	123
159	147
160	148
161	80
162	149
163	150
163	148
164	151
165	152
166	153
167	154
168	26
169	155
169	156
170	157
170	158
171	159
172	160
173	3
174	4
175	68
176	161
177	26
178	162
179	80
180	163
181	164
181	165
181	166
181	167
181	168
181	121
182	169
183	170
184	171
185	3
186	172
197	71
201	89
202	26
186	173
187	174
188	175
189	94
189	155
192	4
190	111
195	176
198	179
191	26
199	180
203	4
193	161
194	80
196	177
200	29
204	181
197	178
205	182
206	183
206	184
207	3
208	185
209	186
210	187
211	188
211	124
212	1
213	189
214	84
215	190
216	191
216	192
217	74
218	4
219	193
220	194
221	195
222	196
223	197
224	198
225	1
226	199
227	200
228	92
229	201
230	202
230	203
231	83
233	204
234	8
235	205
236	206
237	111
238	207
239	176
240	4
241	208
242	16
243	188
244	209
246	210
247	51
248	211
249	212
250	213
251	214
252	215
253	216
254	217
255	218
256	219
257	176
258	26
259	1
260	220
261	221
262	111
262	222
263	164
263	165
263	166
263	167
263	168
263	121
264	161
265	223
266	224
267	71
267	72
268	225
269	226
270	217
271	26
272	227
273	228
274	110
275	229
276	230
276	231
276	15
277	4
278	51
279	176
280	26
281	130
282	131
282	132
283	232
283	233
284	161
285	234
285	235
286	217
287	4
288	236
288	237
289	238
290	146
291	185
292	104
293	239
294	240
295	241
296	242
297	243
298	71
299	13
299	4
299	14
300	201
303	3
304	201
305	244
306	12
\.


--
-- Data for Name: _nc_m2m_tags_events; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms._nc_m2m_tags_events (events_id, tags_id) FROM stdin;
1	1
2	1
3	1
5	1
6	1
8	2
10	2
11	1
12	1
13	2
14	1
15	2
17	1
18	1
19	1
20	2
22	1
23	1
24	1
25	1
26	1
27	1
27	3
28	2
29	1
31	1
32	1
33	3
35	1
36	1
37	1
38	1
41	1
43	1
44	1
45	1
39	1
48	1
50	1
51	1
47	1
49	1
52	1
54	1
53	1
55	1
56	1
57	1
58	1
60	1
61	1
62	1
65	1
66	1
67	1
68	1
69	1
70	1
71	1
72	1
74	1
75	1
77	1
78	1
79	1
80	1
83	1
84	1
87	1
89	1
94	1
97	1
102	1
111	1
112	1
113	1
114	1
116	1
117	1
118	1
120	1
121	1
122	1
123	1
124	1
126	1
127	1
128	1
129	1
130	1
131	1
132	1
133	1
135	1
137	1
138	1
139	1
140	1
136	1
143	1
144	1
59	2
63	3
304	2
303	3
40	2
46	3
81	2
82	2
96	2
101	2
104	2
109	2
115	2
110	2
125	2
134	3
141	2
151	2
164	2
167	2
165	2
166	2
169	2
173	3
178	2
183	2
186	2
189	2
187	2
200	2
204	2
207	2
210	2
213	2
214	2
215	2
216	2
222	2
224	2
226	2
227	2
230	2
232	2
251	2
257	2
292	2
297	2
80	3
105	4
88	4
133	3
161	3
179	3
194	3
207	4
305	5
296	1
295	1
291	1
294	1
293	1
290	1
288	1
285	1
286	1
284	1
283	1
282	1
281	1
280	1
279	1
276	1
278	1
275	1
274	1
273	1
272	1
271	1
270	1
258	1
142	1
145	1
146	1
147	1
149	1
150	1
152	1
153	1
155	1
154	1
158	1
157	1
156	1
159	1
160	1
161	1
162	1
168	1
170	1
171	1
172	1
173	1
175	1
176	1
177	1
179	1
180	1
181	1
182	1
185	1
184	1
188	1
190	1
191	1
193	1
194	1
195	1
196	1
197	1
198	1
199	1
201	1
202	1
203	1
205	1
208	1
219	1
88	5
95	5
103	5
82	5
98	5
105	5
85	5
86	5
92	5
99	5
104	5
90	5
91	5
93	5
96	5
107	5
108	5
106	5
206	5
207	5
209	5
211	5
213	5
212	5
214	5
216	5
217	5
220	5
221	5
223	5
224	5
225	5
226	5
227	5
231	5
230	5
42	2
163	1
229	1
228	1
289	1
306	1
269	1
268	1
267	1
266	1
265	1
264	1
262	1
259	1
260	1
261	1
256	1
254	1
255	1
253	1
252	1
250	1
246	1
247	1
248	1
249	1
241	1
244	1
243	1
242	1
239	1
238	1
237	1
236	1
235	1
234	1
233	1
\.


--
-- Data for Name: about; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.about (id, created_at, updated_at, created_by, updated_by, nc_order, con_name, con_description, website, files) FROM stdin;
1	2025-12-19 01:53:42	2025-12-24 08:36:34	us1wm95uilu3p7mh	us1wm95uilu3p7mh	1	Anthro New England	Howdy, partners! Saddle up for Anthro New England 2026, where we’re moseyin’ on over to the Wild West! Dust off your boots and polish your spurs, and prepare to mingle with fellow furries as we embrace the spirit of the American frontier right here in Boston!	https://ane.boston	\N
\.


--
-- Data for Name: announcements; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.announcements (id, created_at, updated_at, created_by, updated_by, nc_order, title, description, attachment) FROM stdin;
\.


--
-- Data for Name: categories; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.categories (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
7	2025-12-19 01:44:33	2025-12-22 14:22:31	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	7	Gaming
11	2025-12-22 13:58:47	2025-12-22 14:22:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	11	Music/Dance
10	2025-12-19 08:48:42	2025-12-22 14:23:06	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	10	Entertainment
4	2025-12-19 01:37:18	2025-12-22 14:18:28	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	4	Attendee Services
5	2025-12-19 01:39:13	2025-12-22 14:23:31	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	5	Professional/Experienced Hobbyist
9	2025-12-19 08:43:23	2025-12-22 14:23:42	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	9	Main Events
6	2025-12-19 01:42:49	2025-12-22 14:23:47	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	6	Dances
2	2025-12-19 01:32:54	2025-12-22 14:24:09	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	2	Informational/Educational
3	2025-12-19 01:34:50	2025-12-22 14:24:13	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	3	Art
13	2025-12-22 14:04:08	2025-12-22 14:24:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	13	Adult Event (18+)
12	2025-12-22 14:03:44	2025-12-24 22:27:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	12	Adult/Mature Subject
8	2025-12-19 08:35:45	2025-12-24 23:01:10	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	8	Meet Up/Networking
1	2025-12-19 01:29:30	2025-12-22 14:20:15	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	1	Photo
\.


--
-- Data for Name: events; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.events (id, created_at, updated_at, created_by, updated_by, nc_order, name, description, start_time, end_time, locations_id, categories_id, hidden, summary) FROM stdin;
124	2025-12-19 16:23:49	2025-12-24 22:04:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	124	Sports Mascot Fanatics Meet N Greet	Along with animal characters in movies, cartoons, games, and other content, several members of the furry community are also fans of sports mascots. Some of which got into the fandom because of it. And this panel is for those in the furry community who appreciate mascots from the ones they had from their high school and college teams, to major and minor league teams, to global events like the FIFA World Cup. (Remember Zabivaka the Wolf?) Performers, designers, handlers, and fans of sports mascots in general are welcome to join this meet and greet, meet other fellow mascot fans, share stories, share merchandise, and other things related to sports mascots at this panel.	2026-01-17 15:00:00	2026-01-17 15:45:00	9	10	f	\N
1	2025-12-19 01:27:24	2025-12-24 08:25:46	us1wm95uilu3p7mh	us1wm95uilu3p7mh	1	Photo Room	Join us in the Photo Room for an extremely premium photo experience, completely free of charge! All attendees are welcome. We offer digital files and physical prints. (Please visit the ANE website for more details.)	2026-01-15 23:00:00	2026-01-16 05:00:00	1	1	f	\N
25	2025-12-19 08:53:23	2025-12-24 08:30:55	us1wm95uilu3p7mh	us1wm95uilu3p7mh	25	Indie Web Meetup	A meetup for furries interested in handcrafted personal websites! A brief presentation on the Indie Web scene, followed by time to show off your cool websites :3	2026-01-16 16:00:00	2026-01-16 16:45:00	7	8	f	\N
27	2025-12-19 08:56:06	2025-12-24 08:31:27	us1wm95uilu3p7mh	us1wm95uilu3p7mh	27	DIY Leathercraft: Leather Armor	In this panel, we will teach you the basics of how to make leather armor, and leave you with a custom leather bracer made by your own hands. There will be a 30 dollar materials fee, but any are welcome to watch.	2026-01-16 16:30:00	2026-01-16 17:45:00	3	3	f	\N
86	2025-12-19 16:18:17	2025-12-24 22:46:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	86	Punk Furs Meet N Greet	Punk Furs unite and discuss the state of the world, share music, and learn the benefits of being involved in community activism and deconstructing colonialist mindsets. Come mosh, chat and learn!	2026-01-17 01:30:00	2026-01-17 02:15:00	11	8	f	\N
6	2025-12-19 01:39:31	2025-12-24 08:26:47	us1wm95uilu3p7mh	us1wm95uilu3p7mh	6	DIY Leathercraft: Leather Cat Ears	Unleash your inner kitty meow meow! In this panel, we’ll teach the basics of leatherworking, and guide you through making a pair of leather cat ears you can wear home. There will be a 25 USD materials fee, but any are welcome to watch.	2026-01-16 01:30:00	2026-01-16 02:45:00	3	3	f	\N
50	2025-12-19 16:12:45	2025-12-24 21:59:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	50	Sapphic Meet & Greet	A safe space for femme aligned and non-male individuals interested in other femme aligned and non-male individuals to meet and chat with one another.	2026-01-16 21:00:00	2026-01-16 21:45:00	11	8	f	\N
218	2025-12-19 18:34:37	2025-12-22 14:16:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	218	Saturday Night Dances: The Rodeo	Tonight features:<br><br><u>10:00 PM </u>-**CampDisco** - _House and anything in between_<br><u>11:00 PM </u>- **BirdBoy** - _Jackin' House, Garage, Nudisco, Furry Music, etc._<br><u>12:00 AM</u> - **Gojii** - _euphoric multi-genre_<br><u>01:00 AM</u> - **Habs** - _Drum and bass_<br><u>02:00 AM</u> - **Claire vs. Dodge** - _Bass House & Techno_<br><u>03:00 AM</u> - **Goggles** - _Wicked Bass/Electro House_	2026-01-18 03:00:00	2026-01-17 09:00:00	15	6	f	\N
33	2025-12-19 09:04:33	2025-12-24 21:57:52	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	33	Learn to Make a Chainmail Bracelet!	Chainmail can be used to make armor... but it can also be used for some rad accessories! Get an introduction to the basics of chainmail, then make your very own custom bracelet in this paid workshop	2026-01-16 18:00:00	2026-01-16 19:45:00	11	5	f	
51	2025-12-19 16:12:50	2025-12-24 21:59:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	51	Sleepy Stag Suits Meetup/Photoshoot	Photoshoot and meetup spot for Sleepy Stag Fursuiters. 	2026-01-16 21:00:00	2026-01-16 21:45:00	13	8	f	\N
41	2025-12-19 16:11:30	2025-12-24 21:58:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	41	Cooking Furs: Recipe Card Exchange	Bring your favorite recipe to share with other furs that love to cook! Exchange cooking tips and tricks while making new cooking friends! 	2026-01-16 20:00:00	2026-01-16 20:45:00	7	8	f	\N
36	2025-12-19 09:09:07	2025-12-24 08:33:51	us1wm95uilu3p7mh	us1wm95uilu3p7mh	36	This Was In MY Game?! Cut Content in Retro Games!	One of ANE's most popular video game related panels is back for another round! Join Rick Fox, staff member of the popular gaming website "The Cutting Room Floor" on the weird and wild word of cut content and uncommon knowledge in some of your favorite retro games! Even seasoned gamers will be surprised at what lurks in the depths of some of your favorite classics!	2026-01-16 18:30:00	2026-01-16 20:15:00	6	7	f	\N
29	2025-12-19 08:59:22	2025-12-24 08:31:54	us1wm95uilu3p7mh	us1wm95uilu3p7mh	29	How to make things with pretty lights: Making Props with Embedded Lights and Electronics	This session dives into how to cerate props and accessories that shine... literally! We'll dig into 3D Design, 3D Printing, basic electronics, and coding for embedded lights and effects. We'll also cover choosing the right materials, selecting components and powering up safely. Stop by to learn, get inspired, or just to look at the pretty lights!	2026-01-16 17:00:00	2026-01-16 17:45:00	7	2	f	\N
44	2025-12-19 16:12:01	2025-12-24 21:58:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	44	Furry Wheel of Fortune	Cheer and play along as our contestants spin the wheel, buy vowels and solve puzzles in an animal and Furry-themed edition of America's Game, the classic TV game show "Wheel of Fortune"! Watch them spin and win! Contestants will be randomly chosen from the audience. The winners will receive Amazon gift cards and have their winning scores converted into charity money.	2026-01-16 20:00:00	2026-01-16 21:00:00	12	7	f	\N
137	2025-12-19 16:25:22	2025-12-24 22:05:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	137	Bread and Water Soup: The Game	Once upon a time in 2002, a Playstation 2 kaiju battling game called Robot Alchemic Drive was released and would go on to make millions of internet goers laugh for the unintentional memes and jokes it generated. Freedom to direct the plot in ways that can only be described as insanity and "so bad it's good" voice acting make the experience an unexpected comedy. Date a girl who tries to make bread and water soup for your date meal? Mmmm mmm sounds tasty. Flatten this game's version of Walmart because it'll make your anime girlfriend happy? Sure, if that's what the people want! The name of the game in this panel is audience participation with all plot decisions decided by the audience themselves. Are we an angel or a menace? Do we wanna make the game be about robots or a blossoming pro lesbian relationship? The choice is yours and all tied together with commentary from your otter host Talcott!	2026-01-17 17:00:00	2026-01-17 18:15:00	13	7	f	\N
30	2025-12-19 09:00:34	2025-12-24 08:32:12	us1wm95uilu3p7mh	us1wm95uilu3p7mh	30	Fursuit Photo & Parade Line Up	**Please note that the Fursuit Photo is attendance capped this year!**\n\n<br>Please check back later for final details.	2026-01-16 17:00:00	2026-01-16 18:00:00	15	9	f	\N
141	2025-12-19 16:27:04	2025-12-24 22:15:32	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	141	Change the Game - Game Changer in Fursuit!	If you've ever seen Game Changer, the only game show where the game changes every show hosted by Sam Reich and featuring comedians such as Brennan Lee Mulligan, Lou Wilson, and Ally Beardsley, then you'll definitely understand the aim of this panel! To make the game show more local, I, Ryoko Amesapphi, am deciding to bring Game Changer a new twist where our contestants AND our host, myself, are all in fursuit! Our players will have no idea what game it is they are about to play. We'll be playing some of Game Changer's biggest hit episodes, but in accordance with the show, the only way to learn is by playing! The only way to win is by learning, and the only way to begin is by beginning, so without further ado, let's begin!There WILL be moments of audience participation, so please keep your eyes and ears out!	2026-01-17 17:00:00	2026-01-17 18:45:00	10	10	f	\N
162	2025-12-19 16:30:13	2025-12-24 22:40:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	162	Aviation and Aerospace Furs Meetup	Whether professional or hobbiest, meetup with other furs interested in aviation.	2026-01-17 20:00:00	2026-01-17 20:45:00	9	8	f	\N
47	2025-12-19 16:12:23	2025-12-24 21:59:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	47	Photo Room	Join us in the Photo Room for an extremely premium photo experience, completely free of charge! All attendees are welcome. We offer digital files and physical prints. (Please visit the ANE website for more details.)	2026-01-16 20:00:00	2026-01-17 02:00:00	1	1	f	\N
57	2025-12-19 16:13:32	2025-12-24 22:00:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	57	Furry Logic's Escape Room Time Slot 4	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-16 21:30:00	2026-01-16 23:00:00	16	7	f	\N
305	2025-12-24 22:25:56	2025-12-24 22:27:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	305	Hit Me Baby One More Time: Impact 101	An introductory class for beginners or experts wanting to brush up on their knowledge. This class will focus on the basics, including consent, technique, and aftercare. There will be no demos, but come dip your toes into the exciting world of impact play!!	2026-01-17 02:00:00	2026-01-17 02:45:00	6	12	f	\N
28	2025-12-19 08:57:55	2025-12-24 08:31:56	us1wm95uilu3p7mh	us1wm95uilu3p7mh	28	Weird Furry Games for Cool Furry Gamers	Every video game fan loves the Sly Coopers and Bubsy Bobcats of the world, but what if you're too cool for that? Perhaps you've exhausted all the usual furrybait games and you could use a change of pace? The indie scene has plenty of offerings for everyone, and that includes the furry fandom. Come discover new games to play, celebrate some of your favorites, and have a laugh or two along the way!	2026-01-16 16:30:00	2026-01-16 17:45:00	6	10	f	\N
67	2025-12-19 16:14:49	2025-12-24 22:01:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	67	Dragon Meet and Roar!	Come one, come all my fellow Dragon friends!! Come meet and mingle with other dragons! In this all ages and species meet!!Not a Dragon? Come anyways and enjoy the roars from the hall!	2026-01-16 23:00:00	2026-01-16 23:45:00	11	8	f	\N
68	2025-12-19 16:14:56	2025-12-24 22:01:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	68	Fennec Fox Meet & Greet: A-N-SCREE!	A Meet and greet panel/ hang out for Fennec Foxs and admirers. Other species of fox are also welcome to join in the fun.	2026-01-16 23:00:00	2026-01-16 23:45:00	13	8	f	\N
304	2025-12-22 12:29:34	2025-12-24 22:09:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	304	ANE's Furlesque and Drag Show Experience	More information to come soon ;)	2026-01-18 03:00:00	2026-01-18 05:00:00	5	13	f	\N
32	2025-12-19 09:03:14	2025-12-24 08:32:42	us1wm95uilu3p7mh	us1wm95uilu3p7mh	32	Riichi Mahjong	This isn’t your tile stacking computer game! Riichi Mahjong is a 4 player game originating in 19th century China, with later changes in 20th century Japan. Come and learn how to play Riichi Mahjong with other furries!	2026-01-16 18:00:00	2026-01-16 19:45:00	3	7	f	\N
2	2025-12-19 01:30:23	2025-12-24 08:26:00	us1wm95uilu3p7mh	us1wm95uilu3p7mh	2	Panelists Meet and Greet	A Pre-meeting for all panelists to come and be informed about any important information for the weekend of Panels. There will be a Q&A after the presentation for any questions or concerns or special requests that were made. If you are thinking about running an ANE panel in the future or for another con, please feel free to attended. 	2026-01-16 00:00:00	2026-01-16 00:45:00	2	2	f	\N
45	2025-12-19 16:12:08	2025-12-24 21:58:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	45	Furry Logic's Escape Room Time Slot 3	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-16 20:00:00	2026-01-16 21:30:00	16	7	f	\N
72	2025-12-19 16:16:09	2025-12-24 22:01:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	72	Furry Logic's Escape Room Time Slot 5	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-16 23:00:00	2026-01-17 00:30:00	16	7	f	\N
46	2025-12-19 16:12:18	2025-12-24 22:12:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	46	Mini Monster Plushie	Want to make an adorable, fluffy monster plush? This is the panel for you. Kits will be provided with all materials included, 30$ materials cost per kit. Panel space is limited to 25 attendees!	2026-01-16 20:00:00	2026-01-16 21:45:00	9	3	f	\N
167	2025-12-19 16:30:47	2025-12-24 22:16:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	167	So, You Think You Know Zelda?	This is a trivia game show based on the Legend of Zelda. If you're like me and you love the Zelda series, come put that love to the test!	2026-01-17 20:00:00	2026-01-17 21:30:00	12	10	f	\N
286	2025-12-19 18:45:25	2025-12-24 22:34:39	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	286	CRT TV Collecting Safety and Repair	Learn about the analog display technology sweeping the fandom and how you can get your paws on them. We'll look at the best methods for finding CRTs near you, what to use them for, and some expert tips on handling and repair. Who doesn't want a radioactive fuzzy protogen in their living room?	2026-01-18 20:30:00	2026-01-18 21:45:00	9	2	f	\N
155	2025-12-19 16:28:55	2025-12-24 22:39:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	155	Wigglytuff Club: A Pokémon Mystery Dungeon Meetup	Speakers	2026-01-17 19:00:00	2026-01-17 19:45:00	9	8	f	\N
99	2025-12-19 16:19:46	2025-12-24 22:46:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	99	The Outer Limits (18+)	The Outer Limits is Anthro New England's 18+, adult-focused night market style event! All attendees must be 18+ to attend. [More details available here](https://www.anthronewengland.com/events/the-outer-limits/).\n\n_"The headlights of flying cars and monorails racing overhead illuminate a cramped city street. Bright lights and dancing holograms compete for attention all around, but one neon sign shines above the rest: The Outer Limits. You might’ve been nervous for your first time out at this secretive, exclusive club, but as you scan the line of exotic animals in even more exotic clothes in front of you, those nerves turn to excitement. Maybe you’re here to make some new friends, show off a daring outfit, buy a new toy at the shops nearby, or just dance to the best music in the city! Whatever you’re here for, as the bouncer opens the door and waves you inside, the dazzling lights and thumping beats tell the same story: the night is young, and yours to explore…"_\n\n<br>_DJs playing_<br><br>**10:00 PM**MUTTD_NB, Nu Metal Bass_<br>**11:00 PM**XXBUCKDOGXX_Breakbeat_<br>**12:00 AM**Cliffjump_breaks, acid techno, tech trance, psy trance_	2026-01-17 03:00:00	2026-01-17 06:00:00	17	13	f	\N
220	2025-12-19 18:36:40	2025-12-24 22:49:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	220	The Good Puppy Panel 2	Are you a pup, handler, or other kind of person? Then come to The Good Puppy Panel... 2! Unlike a mosh or a Pet Play 101, this panel takes you through a comedic and slightly unhinged presentation about how to be a good puppy! Marble Pupper, world's dumbest puppy, returns to teach you what it takes to be a puppy as good as them. This year's panel is new and improved, so whether you attended last year or not, you'll get a whole new experience and become an even better puppy than you are now! Pet players unite for what Marble claims will be "the best panel ever again."	2026-01-18 03:30:00	2026-01-18 04:45:00	2	10	f	\N
248	2025-12-19 18:40:58	2025-12-24 23:04:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	248	Gender and Jujutsu Kaisen	Breaking out of the Prison Realm of gender norms: An in-depth analysis of the utilization of masculine and feminine characteristics in Gege Akutami's popular series: Jujutsu Kaisen. Breaking down our favorite characters' gender expression, and what roles gender plays in Jujutsu society.	2026-01-18 16:00:00	2026-01-18 16:45:00	8	2	f	\N
74	2025-12-19 16:16:32	2025-12-24 22:01:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	74	Shedding Inhibitions: A Guide to Public Fursuiting	So you own a fursuit, know how to care for it and are having a blast with it at conventions? Great! Perhaps you've also considered bringing your fursona out into everyday areas, but the idea makes you uneasy, uncertain or insecure. This is very common, and understandably so - its a big step!I'm Ferdinand, a ferret with fifteen years of public fursuiting under my belt, and I'd like to pass my experience and hard-won nuggets of knowledge down to you. In this informative panel, learn the important differences between con suiting and public suiting, as well as:- where to go and not to go- what to bring- proper etiquette with the general public- when to interact(and when to steer clear!)- keeping your suit safe...and more!I hope you'll join me, and learn if public fursuiting is right for you!	2026-01-16 23:30:00	2026-01-17 00:45:00	7	2	f	\N
217	2025-12-19 18:34:28	2025-12-24 22:49:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	217	CrackerJackBox Games	Come play Jackbox Games with Crackerjack and crew! Players revolve for each game and are randomly drawn, we will play until time runs out. Prizes to be awarded, TeeKO Shirts to be impulsively bought, and hilarity to ensue!	2026-01-18 03:00:00	2026-01-18 04:45:00	8	7	f	\N
278	2025-12-19 18:44:32	2025-12-24 22:35:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	278	Fly Fishing Fundamentals for Fly-Curious Furries	A basic introduction to the sport of fly fishing, covering basic equipment, techniques, fly selection and water reading skills	2026-01-18 19:30:00	2026-01-18 20:15:00	7	2	f	\N
147	2025-12-19 16:27:35	2025-12-24 22:38:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	147	Wind Traders	Come to Wind Traders to trade all sorts of goodies with others! We allow trades of all kinds from enamel pins to fursuits, stuffed animals to toys, and anything else you may want to trade! Please keep items SFW as this is an all ages setting!	2026-01-17 18:00:00	2026-01-17 19:45:00	3	8	f	\N
83	2025-12-19 16:17:49	2025-12-24 22:02:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	83	Furry Logic's Escape Room Time Slot 6	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 00:30:00	2026-01-17 02:00:00	16	7	f	\N
289	2025-12-19 18:45:44	2025-12-24 22:59:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	289	Western Hip-Hop 101	Ever wanted to learn how to dance a little bit, and show it off to your friends at the local saloon? Come learn basic hip hop moves with an experienced dinosaur! Based off the teachings of my local dance studio, The Hive, this will be a fun, lowkey, crash course dance class taught for the first time by me! We'll see what comes out of this session but I'm sure by the time the panel ends you'll know how to groove, at least a little bit! :3 Yeehaw!	2026-01-18 21:30:00	2026-01-18 22:15:00	5	11	f	\N
294	2025-12-19 18:46:27	2025-12-24 22:33:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	294	The Hope Panel - PCD & Starting Your Next Big Adventure	Feeling down about the end of the con? Come take stock of an amazing weekend and use proven techniques to beat post-con depression and start your next great adventure!	2026-01-18 21:30:00	2026-01-18 22:45:00	7	2	f	\N
231	2025-12-19 18:38:23	2025-12-24 22:49:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	231	Littles Coloring Night (18+)	Come grab a free coloring sheet and color with friends in this community	2026-01-18 05:30:00	2026-01-18 06:45:00	3	3	f	\N
285	2025-12-19 18:45:18	2025-12-24 22:34:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	285	Pokemon Play, Trade, and Social Hour!	This panel creates a space for pokemon fans to meet, battle, and trade! All levels of interest are welcome. Bring your game consoles, cards, or just yourself, and join us as we hang out and catch 'em all!	2026-01-18 20:30:00	2026-01-18 21:45:00	11	7	f	\N
14	2025-12-19 08:39:20	2025-12-24 08:28:44	us1wm95uilu3p7mh	us1wm95uilu3p7mh	14	Kinz Chat	Welcome to the Webkinz Clubhouse! Are you an avid webkinz player, reminising your childhood account, or maybe just interested in starting a new account? Come join us to talk everything kinz, start a new account, trade plush, or maybe even win a free pet!	2026-01-16 15:00:00	2026-01-16 15:45:00	10	7	f	\N
42	2025-12-19 16:11:37	2025-12-24 22:53:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	42	Musician/Audio Furry Meetup	Calling all musicians and anyone audio-adjacent--I want to hear you! Come here to meet like-minded furry creatives who work with audio in various forms and share our common musical interests, passions, and open the door to collaborations and friendships. Singer, instrumentalist, audio engineer, foley artist, A&R rep, acoustics, designer--we want to meet all of you. We'll mingle, have some ice-breakers, and make sure we all keep in touch during and after ANE!	2026-01-16 20:00:00	2026-01-16 20:45:00	13	8	f	\N
281	2025-12-19 18:44:57	2025-12-24 22:35:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	281	Sea Shanties!	No talent or shanty knowledge required! Please come to sing shanties, share shanties, or just enjoy them! Feel free to join our telegram channel at https://t.me/+ARddu0W9cKlkOWRh to submit lyrics in advance!(not required)	2026-01-18 20:00:00	2026-01-18 21:45:00	2	11	f	\N
91	2025-12-19 16:18:50	2025-12-24 22:46:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	91	Photo Room (18+)	Join us for an exclusive after-hours opening of the Photo Room! We welcome your spicy outfits and scandalous poses! 18+ only. ID is required. (Please visit the ANE website for more details.)	2026-01-17 02:00:00	2026-01-17 05:00:00	1	1	f	\N
306	2025-12-24 22:59:49	2025-12-24 23:01:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	306	Tokusatsu Furs	Rangers, Robots, and Rodan, oh my! All tokufurs ride as this meetup is dedicated to furries who are fans of Japanese live action special effect shows/movies, or Tokusatsu! Whether its the rampaging Godzilla and his kaiju rivals, or the mighty morphin Power Rangers and their Japanese counterpart Super Sentai, this emetup welcomes all both small and ULTRA sized. Come on down to discuss your favorite movies, series, and HENSHIN! with new friends and series!  (Please keep Megazords and Rider Bikes parked outside the venue)	2026-01-18 20:30:00	2026-01-18 21:15:00	6	8	f	\N
213	2025-12-19 18:33:46	2025-12-24 22:48:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	213	What the Fat? - A Sociological Perspective to Fatfurs	Ever wondered how this whole fatfur/inflation thing happens sociologically? Join me in my comedic presentation about the sociological development of why we like this stuff! From studies, to media exposure; we take on the big question of, "Why am I into this stuff?"(There's also a Kahoot at the end for a special prize)\r	2026-01-18 02:30:00	2026-01-18 03:15:00	11	12	f	\N
118	2025-12-19 16:23:02	2025-12-24 22:04:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	118	Film Industry Furs: Cinema Camera Building	As a gear head for camera tech as well as a passionate creative, I'd like to run a panel on the basics of cinema camera building and the jobs of the camera, grip, and electrical departments on professional film sets. I'd like to feature a breakdown of what each part of a cinema camera rig does and why its important to us film makers for achieving the vision of a project. This would ideally include me renting a cinema camera like an ARRI and a decent lens, pairing this with my personal accesorry kit I'd want to demonstrate how to build out a camera for film set use and best practice for handling these wildly expensive but amazingly useful artistic tools. I'd also like sprinkle in bits of information about on set etiquette and structure. I'd seek to give insight into the specifics of on set roles and what being a freelancer in the film world is like. That bit might sound boring but I want to give my furry friends who are interested in film work some honest advice. Hopefully I can communicate (without too much cynicism) what the industry is really like, what I have learned works for me, and what I know not to do.	2026-01-17 14:00:00	2026-01-17 15:15:00	7	5	f	\N
266	2025-12-19 18:43:13	2025-12-24 23:02:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	266	Furry Scrabble	It's the crossword game you've played all your life, but never quite like this! Join host Randy Ringtail as he brings the classic '80s game show to Anthro New England! Watch as contestants are drawn from the audience to test their crossword skills and win great prizes!	2026-01-18 18:00:00	2026-01-18 19:00:00	12	10	f	\N
100	2025-12-19 16:19:54	2025-12-22 14:04:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	100	Friday Night Dances: DIGIFEST	Performers to be announced soon	2026-01-17 03:00:00	2026-01-17 09:00:00	5	6	f	\N
148	2025-12-19 16:27:37	2025-12-22 14:09:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	148	Animal Music Concert	Where ANE rocks the house!<br><br>Final lineup of bands performing to be announced soon.	2026-01-17 18:00:00	2026-01-17 21:00:00	15	9	f	\N
165	2025-12-19 16:30:34	2025-12-24 22:16:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	165	Let's Play a Dating Sim!	Have you ever wanted to date Shadow The Hedgehog? What about Santa? Chicken nuggets? Join us as we enter the weird and wacky world of dating sims. 	2026-01-17 20:00:00	2026-01-17 21:15:00	3	7	f	\N
250	2025-12-19 18:41:08	2025-12-24 23:04:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	250	Dance Battle 101 (Electric Boogaloo)	Dance Battle 101 is back! Come join our amazing judges, new and veteran dancers, and experienced dance teachers to learn a crash course of the tips, tricks, and basics of Dance Battles! (This also includes a INTERACTIVE portion-perhaps you may find yourself in a dance battle yourself!)	2026-01-18 16:00:00	2026-01-18 16:45:00	5	11	f	\N
17	2025-12-19 08:43:39	2025-12-24 08:29:29	us1wm95uilu3p7mh	us1wm95uilu3p7mh	17	C2 3DS StreetPass Meetup	This is a space for all fans of StreetPass to meet one another and exchange StreetPass data! Come get your puzzle pieces, Find Mii warriors, and maybe some new friends!	2026-01-16 15:00:00	2026-01-16 16:00:00	12	8	f	\N
34	2025-12-19 09:06:46	2025-12-24 08:33:21	us1wm95uilu3p7mh	us1wm95uilu3p7mh	34	Fursuit Parade	The Fursuit parade begins immediately following the Fursuit Photo. **Please note that the photo is attendance capped, meaning if you do not make it into the Photo, you will be asked to wait until the parade has started and you are able to enter the parade.**	2026-01-16 18:00:00	2026-01-16 20:00:00	17	9	f	\N
19	2025-12-19 08:46:17	2025-12-24 08:29:51	us1wm95uilu3p7mh	us1wm95uilu3p7mh	19	Entering the Equine World: Horse ownership and 101 on how to enter this world	Howdy! Have you ever had an intrest in joining the equine community (yes the actual animal 🤣🥰), or are you already apart of it and want to either own a horse one day or want to meet your fellow furry riders. Well this is the perfect panel for you. Join me Speedy on the basic rundowns of horse ownership and how to enter this lovely community and world of working with these four legged companions. This panel is great for those who have been riding for years to meet other furries who do, while also welcoming those who need a place to start!	2026-01-16 15:00:00	2026-01-16 16:15:00	13	5	f	\N
18	2025-12-19 08:45:05	2025-12-24 08:29:35	us1wm95uilu3p7mh	us1wm95uilu3p7mh	18	Bringing Characters to Life - Tips on Costumed Performance	Have an awesome costume but aren't sure what to do next? Want to breathe some life into your performances? Come and hear the top tips from seasoned pro Raeburn. (Think Fursuit 201).	2026-01-16 15:00:00	2026-01-16 16:15:00	6	2	f	
122	2025-12-19 16:23:33	2025-12-24 22:04:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	122	Fursona Lore Zinemaking	What's your fursona's origin story? Join illustrator, zinemaker, & printmaker pup Just Right in this paws-on zinemaking workshop where you create a minizine all about how your fursona came to be! Materials will be provided, but feel free to bring your favorite art supplies! 	2026-01-17 14:30:00	2026-01-17 15:45:00	3	3	f	\N
238	2025-12-19 18:39:21	2025-12-24 23:05:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	238	No Mud, No Lotus: Buddhism 101	Lets face it: The modern world feels like it's on fire and we as humans struggle to stay afloat while feeling helpless to empower change of what is occurring around us. But, what if change could begin from within one's own heart and mind, and expand outwards towards others to help make the world a kinder place for all?<br><br>Join Riiya, a fellow furry and genyen (vow-holding layperson) in the Gelug lineage and Drikung Kagyu lineage of Tibetan Buddhism as she touches briefly on how to incorporate aspects of traditional teachings into daily life, regardless of one's spiritual or religious background! We will start the session with a breathing exercise, followed by discussing what is known in Buddhism as the "Four Noble Truths" and the "Eightfold Path" and how they can help shift the one's mind towards compassion and understanding in a world filled with turbulence and negativity. We will close out with a "Calm Abiding" style meditation session. All are welcome, bring your friends and come as you are :)<br><br>Sensory note: a mid-pitch standing bell will be rung to signal the beginning and end of the meditation session!	2026-01-18 14:00:00	2026-01-18 15:15:00	7	2	f	\N
135	2025-12-19 16:25:11	2025-12-24 22:05:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	135	Trinket Trade!	Hi hello!! Looking to meet some new friends and give out freebies? Come to our Trinket Trade and trade kandi, perlers, stickers, 3D printed doodads, trading cards, party favors you got at the grocery store, or whatever you want really!!	2026-01-17 16:30:00	2026-01-17 17:15:00	11	8	f	\N
234	2025-12-19 18:38:53	2025-12-24 23:06:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	234	Wildfire Yoga - Quadrobics	Join me, Awe Tiger, for a 3 part journey into yoga, meditation, and functional movement theory.  Each day will have a different focus - Day 1: Hip and Knee Health.  Day 2: Shoulders, Spine and Breath.  Day 3: Paws and Purrception - Quadrobics Conditioning.  Come explore these traditional health and spirituality based eastern practices meant to free one to the possibility of living in alignment with nature and unleashing the power of their animal body.  All classes will offer options for those with limited mobility who wish to practice from a chair. 	2026-01-18 13:00:00	2026-01-18 14:45:00	8	2	f	\N
123	2025-12-19 16:23:38	2025-12-24 22:04:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	123	3D Modeling, Animation, and VF(ur)X	Have you ever been interested in 3D modeling, VFX, or animation (keyframe or motion capture)? This panel aims to give you an overview of the programs, terms, and processes you will work with in your VF(ur)X journey! 	2026-01-17 14:30:00	2026-01-17 15:45:00	13	2	f	\N
183	2025-12-19 16:33:07	2025-12-24 22:17:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	183	Muscle Furs Meetup	Do you like furries with big muscles? Do you wanna see them get bigger? Join us at the Muscle Furs Meetup and get to know your fellow muscle enthusiasts! Real-life muscles not required, all genders and species welcome!	2026-01-17 22:00:00	2026-01-17 22:45:00	8	8	f	\N
292	2025-12-19 18:46:01	2025-12-24 22:20:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	292	YouTube Poops 101 (Not the Dirty Kind)	\N	2026-01-18 21:30:00	2026-01-18 22:45:00	13	10	f	\N
223	2025-12-19 18:37:08	2025-12-24 22:49:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	223	Free Will, Individuality, and Other Things... You Don't Need to Worry About	For anyone who has a passing interest in drone kink, but is made uncomfortable by the dark themes that often accompany it like loss of identity and corporate control. This drone themed critique of individualism aims to present a more meaningful approach to the kink that focuses on reframing the self as part of a community.	2026-01-18 04:30:00	2026-01-18 05:15:00	7	2	f	\N
277	2025-12-19 18:44:29	2025-12-22 14:21:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	277	Variety Show	Come on down to Madame Camille's saloon stage, where she'll introduce you to a cornucopia of delights! The best of the West will be performing just for you: singing, dancing, comedy, magic, drag... you name it, she's got it! So grab your rhinestone cowboy hat and dancing spurs, then mosey on over for the show with the most Variety this town's ever seen!	2026-01-18 19:00:00	2026-01-18 21:00:00	5	9	f	\N
4	2025-12-19 01:35:11	2025-12-24 08:25:07	us1wm95uilu3p7mh	us1wm95uilu3p7mh	4	Registration (Thursday)	\N	2026-01-16 00:00:00	2026-01-16 03:30:00	4	4	f	\N
239	2025-12-19 18:39:32	2025-12-24 23:05:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	239	Bugtography	Take an up-close look into the unseen world of invertebrate photography with invertebrate zoologist and hobbyist photographer Fedora Fennec. See how camera technology has advanced to allow us to see this hidden world in new ways, learn about some of the unique invertebrates that inhabit the Northeast, find out about recent and relevent conservation issues and how you can help, or just come for some intersting photography!	2026-01-18 14:00:00	2026-01-18 15:45:00	13	2	f	\N
284	2025-12-19 18:45:10	2025-12-24 22:34:52	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	284	Yarn Crafts Meetup	A meetup for fans of all things yarn. Whether you knit, crochet, embroider, or just like playing with balls of yarn... Come show off your projects, or work on something you've brought!	2026-01-18 20:30:00	2026-01-18 21:15:00	13	8	f	\N
97	2025-12-19 16:19:35	2025-12-24 22:03:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	97	Goat Screamup	I scream, you scream, we all scream* because we’re goats!* subject to noise regulations	2026-01-17 03:00:00	2026-01-17 04:15:00	8	8	f	\N
140	2025-12-19 16:25:42	2025-12-24 22:05:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	140	Furry Logic's Escape Room Time Slot 7	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 17:00:00	2026-01-17 18:30:00	16	7	f	\N
110	2025-12-19 16:22:10	2025-12-24 22:15:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	110	Powerpoint Night	YOU MAKE A POWERPOINT WHICH IS FUNNY AND COOL AND AWESOME AND IF WE LIKE IT AND EVERYONE LOVES IT U PRESENT IT OKAY? ITS SIMPLE GWAH :3	2026-01-17 06:00:00	2026-01-17 07:15:00	6	10	f	\N
156	2025-12-19 16:29:31	2025-12-24 22:39:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	156	Hyena Meat and Greet	Speakers	2026-01-17 19:00:00	2026-01-17 20:15:00	11	8	f	\N
194	2025-12-19 16:34:35	2025-12-24 22:43:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	194	Let' $ew! Squeaky Tacos	$10 per student.Beginner skill level.13+ or bring a parent or guardian, please!The pattern is easy. It's just like a real taco! Start with a circle. Fill it with 'meat,' add a squeaker, and garnish as you see fit. (Ohgosh don't eat these!!!)	2026-01-17 23:30:00	2026-01-18 01:15:00	3	3	f	\N
224	2025-12-19 18:37:26	2025-12-24 22:49:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	224	Yiff Bingo	Come compete against your friends in an explicit furry-themed game of Bingo. Prizes will be given for the winners, featuring one-of-a-kind NSFW prints from some of your favorite artists! This panel is for those 18 years of age and older.	2026-01-18 04:30:00	2026-01-18 05:45:00	11	12	f	\N
271	2025-12-19 18:43:44	2025-12-24 22:36:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	271	Furry Logic's Escape Room Time Slot 14	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-18 18:30:00	2026-01-18 20:00:00	16	7	f	\N
192	2025-12-19 16:34:24	2025-12-22 14:13:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	192	Fursuit Dance Competition	As one of our most attended and watched events, the fursuit dance competition is a cornerstone of Anthro New England! Talented individuals don their fursuits and strut their stuff – to music! Those who pass prelims are invited to perform live in our ballroom before our attendees and our panel of judges for feedback and criticism.<br><br>Come join us live to see the best performers for ANE 2026!<br><br>Interested in performing? Applications are still open for the Dance Competition until January 10th!<br><https://www.anthronewengland.com/events/dance-competition/>	2026-01-17 23:00:00	2026-01-18 01:00:00	15	9	f	\N
287	2025-12-19 18:45:35	2025-12-22 14:22:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	287	Fursuit Recess	\N	2026-01-18 20:30:00	2026-01-18 22:30:00	15	9	f	\N
8	2025-12-19 01:43:07	2025-12-24 08:26:57	us1wm95uilu3p7mh	us1wm95uilu3p7mh	8	ANE Department Vs. Department	Want to see what happens when ANE departments play Jackbox and be in the audience to help sway which Department is the funniest around to be Champs of ANE '26 Most Rootin Tootin Department.	2026-01-16 03:00:00	2026-01-16 05:45:00	6	7	f	
200	2025-12-19 18:31:37	2025-12-24 22:18:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	200	ANE Poetry Slam!	Furry poets, come hither! Come share your art of the spoken and written word with other fellow poets and poetry lovers! Rhyme or free verse, iambic pentameter to the page as a canvas, your own poetry or poems from a poet you really enjoy, it doesn't matter! Share poems in a welcoming space with other poets.	2026-01-18 00:30:00	2026-01-18 01:45:00	13	3	f	\N
245	2025-12-19 18:40:31	2025-12-22 14:19:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	245	Super-Sponsor, Patron, & Donor Brunch	We couldn't do this without you! As a heartfelt thank you, we’re hosting a private brunch for you catered by the Westin! Chit-chat with your fellow Patrons, Donors and Super-Sponsors, and enjoy the delicious breakfast provided by our wonderful hotel.<br><br>**Beverages**<br>Freshly Brewed Starbucks Coffee (Regular/Decaf)<br>Selection of Tazo Teas<br>Chilled Orange, Grapefruit & Cranberry Juice<br>Whole & Non-Fat Milk<br>Add-ins: Cream, Almond & Oat Milk, Lemon, Clover Honey, Sugar (Three-ways)<br><br>**Mains** <br>Cage-free Scrambled Eggs with Cabot Cheddar & Green Onions<br>Thick-sliced Maple Peppered Bacon<br>North Country Smoked Maple Sausage Links<br>Farmhouse Sweet Potato Hash with Seasonal Vegetables<br><br>**Baked Goods**<br>Seasonal Scones with Apricot Preserves<br>Assorted MA Local Bagels served with Cream Cheese and Vermont Butter<br><br>**Fruit & Yogurt**<br>Sliced Seasonal Fresh Fruit & Berries<br>Cranberry Apple Parfait with House Granola, Greek Yogurt & Local Honey	2026-01-18 15:00:00	2026-01-18 16:30:00	10	9	f	\N
202	2025-12-19 18:32:01	2025-12-24 22:43:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	202	Furry Logic's Escape Room Time Slot 12	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-18 00:30:00	2026-01-18 02:00:00	16	7	f	\N
117	2025-12-19 16:22:58	2025-12-24 22:04:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	117	Western MA Furs	Do you like meeting people? Do you like meeting people who also live in the same area as you? Well, if you said yes, come on down and meet some of your fellow Western MA Furs! We host meets once a month and do a ton of cool events! Come learn and hang out with us! 	2026-01-17 13:30:00	2026-01-17 14:15:00	6	8	f	\N
128	2025-12-19 16:24:17	2025-12-24 22:04:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	128	Pro Wrestling Furs!	A panel for fans of pro wrestling! We'll discuss our favorite wrestlers and watch some matches! 	2026-01-17 15:30:00	2026-01-17 16:45:00	6	2	f	\N
126	2025-12-19 16:24:05	2025-12-24 22:04:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	126	The Martian Trail	It's a race to Mars! Inspired by "The Oregon Trail" this event will be a card-based adventure meant to show the challenges of spaceflight. Join us as we talk about what it will take to get to the red planet, then play as one of 4 competing space agencies in a race to see who can get to Mars...alive! 	2026-01-17 15:00:00	2026-01-17 16:15:00	11	7	f	\N
130	2025-12-19 16:24:37	2025-12-24 22:05:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	130	Herpetology 101 and Meet and Greet	Come meet some not so creepy crawlies and learn about their importance in our world!	2026-01-17 16:00:00	2026-01-17 16:45:00	13	2	f	\N
16	2025-12-19 08:42:12	2025-12-24 08:29:12	us1wm95uilu3p7mh	us1wm95uilu3p7mh	16	Opening Ceremonies	Let's kick this Western off right! Come join Anthro New England's two co-chairs, Atlas and Flint as they officially open ANE for 2026! A little birdy told us there may be a surprise! 	2026-01-16 15:00:00	2026-01-16 16:00:00	5	9	f	\N
258	2025-12-19 18:42:16	2025-12-24 22:36:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	258	Furry Logic's Escape Room Time Slot 13	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-18 17:00:00	2026-01-18 18:30:00	16	7	f	\N
154	2025-12-19 16:28:24	2025-12-24 22:39:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	154	Magic the Gathering: Commander Meetup!	A general Magic the Gathering Commander meetup and anyone is welcome to drop in and play a game! Any beginner or experienced player is welcome to join in to play a game!	2026-01-17 18:30:00	2026-01-17 20:00:00	12	7	f	\N
299	2025-12-19 18:47:09	2025-12-22 14:23:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	299	Closing Ceremonies	It's that time when we're all tired, and no one really wants the con to end, but everything good has to come to a close! Come join ANE's co-chairs, Flint and Atlas to wrap things up, talk about numbers, and maybe put on a good ol' movie. 	2026-01-19 00:00:00	2026-01-19 01:00:00	5	9	f	\N
195	2025-12-19 16:34:44	2025-12-24 22:43:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	195	Unconventional Sonas Meet-Up	The world is full of diverse species so the possiblities for fursonas are endless! As someone who originally pnly has a horseshoe crab sona I often did not fit into a lot of the meet-ups for sonas so decided to make my own! Maybe you have an unsusual hybrid, a sepcies few have done before, or you created your own unique creature, well then come meet others with uncoventional sonas!	2026-01-18 00:00:00	2026-01-18 00:45:00	2	8	f	\N
178	2025-12-19 16:32:13	2025-12-24 22:17:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	178	The Flannel Panel	Do you know what a Cowboy and YOU have in common?? THATS RIGHT, YOU BOTH WORE PLAID!!!! Come on down to the panel hosted by a "bunch of idiots" as we ask the most important question of the Convention.. "is it a cult yet?"	2026-01-17 21:30:00	2026-01-17 23:15:00	6	10	f	\N
174	2025-12-19 16:31:50	2025-12-22 14:11:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	174	Fursuit Games	**_Fursuiters, please show up 15 minutes early to get into the games!_**<br><br>Welcome on, and all - it's time for the Wild West's Fursuit Games!	2026-01-17 21:00:00	2026-01-17 23:30:00	5	9	f	\N
229	2025-12-19 18:38:01	2025-12-24 22:57:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	229	Karaoke	Dust off your boots and grab the mic for some Karaoke! Join us for an evening of music and mayhem—sing your heart out, enjoy the show, and make some noise with the crowd.	2026-01-18 05:00:00	2026-01-18 08:00:00	10	11	f	\N
153	2025-12-19 16:28:18	2025-12-24 22:39:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	153	Furry Logic's Escape Room Time Slot 8	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 18:30:00	2026-01-17 20:00:00	16	7	f	\N
296	2025-12-19 18:46:46	2025-12-24 22:33:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	296	How to Run Your Own Furmeet	Have you ever wanted to start your own furmeet? Do you want to start a furbowl? Perhaps a small get together or even a gaming meet? In this panel, you will learn the ins and outs of running a furmeet from an experienced host of various furmeets, including a successful furbowl. You will learn how to schedule meets, find a venue, manage the furmeet itself, and create good relations with the staff of your venue along with various tips on how to make your furmeet successful.	2026-01-18 22:00:00	2026-01-18 22:45:00	9	2	f	\N
197	2025-12-19 16:35:02	2025-12-24 22:43:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	197	Meet the Charity: Queer Scouts	Let's welcome back Queer Scouts for their second year as our charity! They'll tell us more about their work while teaching us how to make terrariums. (25 attendees max, be sure to arrive early!)	2026-01-18 00:00:00	2026-01-18 01:15:00	9	2	f	\N
201	2025-12-19 18:31:46	2025-12-24 22:43:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	201	Behind the Screens! AKA: Rito Yells at You About His Insanity	Come to a panel where rito (me) tells you about how the screen animations were made with blender, spite, and more ambition than sense.	2026-01-18 00:30:00	2026-01-18 01:45:00	10	2	f	\N
241	2025-12-19 18:39:42	2025-12-24 23:05:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	241	Turn Your FURsona into a TROLLsona	Do you miss the Homestuck fandom or do you want to experience it in the safety of a furcon?? Come join us to turn your fursona into a trollsona and get a little creative with species swapping! All the art supplies will be supplied including laminators and hole punch to make a badge of them! 	2026-01-18 14:30:00	2026-01-18 15:45:00	6	3	f	\N
237	2025-12-19 18:39:16	2025-12-24 23:05:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	237	Splatoon Meetup	Grab your Inkzookas and chug some Tacticoolers, because it's Splatoon meetup time! Bring your Nintendo Switch (or Switch 2) and join us for some local multiplayer gaming. You can also submit clips ahead of time, which we'll show during the panel so everyone can see the team kill you pulled off with just a Splattershot Jr. and some grit. Submit your clips here: https://splatoonfcl.morbinti.me	2026-01-18 14:00:00	2026-01-18 14:45:00	3	7	f	\N
23	2025-12-19 08:50:56	2025-12-24 08:30:26	us1wm95uilu3p7mh	us1wm95uilu3p7mh	23	Next Level Tips and Tricks For Fursuit Travel	Traveling to a con with a new fursuit? What about two suits, or three, or four? In this panel Equus Silvermane will discuss budget friendly techniques to transport your furry gear to a con and keep it organized once your are there. From packing bags for a flight, to loading your car for a road trip and setting up the hotel room at the other end, this panel will help you be your best furry self wherever your destination may be.	2026-01-16 16:00:00	2026-01-16 16:45:00	2	2	f	\N
11	2025-12-19 08:34:26	2025-12-24 08:27:38	us1wm95uilu3p7mh	us1wm95uilu3p7mh	11	Stick(er) around! Sticker Trading and Sharing	Hey! Do you have some stickers you want to share with people? Maybe you're wondering why people have so many stickers? Come on by and join in one of the coolest panels in the con. Even if you don't have stickers, stop on by!	2026-01-16 14:30:00	2026-01-16 15:45:00	7	8	f	\N
107	2025-12-19 16:21:27	2025-12-24 22:47:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	107	Tails of the Trip, South of the Border	Howdy there! Pull up a stump and grab your can of beans for some of the strangest stories this side of the Mississippi. From rigor mortis rig hands the revolver wielding road rage. So hang on to your hat and keep that snake oil handy!  This is tails for the trip!	2026-01-17 05:00:00	2026-01-17 05:45:00	7	10	f	\N
252	2025-12-19 18:41:27	2025-12-24 23:04:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	252	Clean Your Beans! Keeping Your Fursuit Looking Fresh	We all talk about the importance of the care and keeping of your fursuit, but how exactly do you do that? Tired of being a musky husky? I'm here to share all the tips and tricks I've learned so far, from basic care to what to do when disasters happen. I'll also go over the non-washing aspected of fursuit care, like brushing and heat treating. It may not be easy looking this fluffy, but it doesn't have to be painful!	2026-01-18 16:00:00	2026-01-18 05:15:00	7	2	f	\N
150	2025-12-19 16:27:49	2025-12-24 22:38:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	150	Demystifying Protogen Building: You Too Could Be a Toaster!	Have you ever looked at a protogen head and wondered how they're made? Or have you looked before, but aren't sure where to start? This panel will provide you with some background on the design, construction, wiring, and coding of the average toaster as well as provide a repository of knowledge to use if you decide to make your own!	2026-01-17 18:30:00	2026-01-17 19:30:00	8	2	f	\N
269	2025-12-19 18:43:31	2025-12-24 23:01:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	269	The Joy of Problem Solving in Automation Games	Maybe my hyperfixation can be your next hyperfixation too! There are many good automation games out there and I'd like to talk to you about some of them and why they are some of the most satisfying video games I've had the pleasure to play. Factorio, Satisfactory, Shapez2 and Mindustry will be mentioned, and there are other games that are worth mentioning as well	2026-01-18 18:30:00	2026-01-18 19:30:00	7	7	f	\N
297	2025-12-19 18:46:57	2025-12-24 22:20:52	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	297	Computers are Dumber than You Think!	Technology is intimidating! A lot of people think that working with modern technology is all super intelligent people working in cool well run data centers with everything operating as a well oiled machine. The truth is, computers are WAY dumber than you think, and so are some of the people who maintain them.Hear inside stories about how the industry really works. Everything from stupid syntax errors to spreadsheet hell, misconfigurations and dumb decisions. Everything is ALWAYS on fire and global tech infrastructure is hanging by a thread. Come learn firsthand why most companies schedule change freezes during major furry conventions. You might just learn something while you're at it.	2026-01-18 22:00:00	2026-01-18 23:15:00	2	5	f	\N
290	2025-12-19 18:45:51	2025-12-24 22:34:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	290	Fursuit Figure Drawing	A life drawing panel with fursuiter models.  Attendees should plan to bring their own supplies, though a limited quantity will be provided.  Please note, this is panel is intended as an opportunity to practice drawing skills with live subjects instead of active instruction on how to draw.  	2026-01-18 21:30:00	2026-01-18 22:45:00	3	3	f	\N
283	2025-12-19 18:45:04	2025-12-24 22:35:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	283	Pizza Panel in Space	Join us for a fun adventure of space knowledge where we explore interesting oddities such as space pizza delivery, space soda, that time space was a whole country, and SO MUCH MORE! Pizza not included.We will be hosting a brief show and tell at the end, so if you have any space oddities, merch, or ANYTHING space related, please bring it and show it off, we want to see ALL OF IT	2026-01-18 20:30:00	2026-01-18 21:15:00	7	2	f	\N
254	2025-12-19 18:41:57	2025-12-24 23:03:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	254	Being and Fluffiness	Do you want to feel like you're in college but for free? Do you want to balance your fun con day out with a lecture on the existentialist origins of furry identity? Let's put on our tweed jackets and discuss the philosophy of being animal people!	2026-01-18 16:30:00	2026-01-18 17:45:00	3	2	f	\N
273	2025-12-19 18:44:02	2025-12-24 22:36:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	273	TikTok Furry Meetup	A place for creators on TikTok, YouTube Shorts, and Instagram reels to meet up and film! Bring your buisness cards / stickers to help get your handle out there. 5 Minutes In:Group Photo 10 Minutes In:Group VideoRecord with other people and build connections through our love of content creation. All ages welcome, fursuiters and non-fursuiters welcome.	2026-01-18 19:00:00	2026-01-18 19:45:00	10	8	f	\N
173	2025-12-19 16:31:41	2025-12-24 22:41:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	173	Armor Academy: Chainmail Plushie	In this panel, we will teach how to make an entire shirt of chainmail – scaled down to plushie size! Bring your own plushie, or use one of the stuffed animals provided. There will be a 30 dollar materials fee to participate, but any may observe for free.	2026-01-17 21:00:00	2026-01-17 22:15:00	9	3	f	\N
240	2025-12-19 18:39:35	2025-12-22 14:18:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	240	Registration (Sunday)		2026-01-18 14:00:00	2026-01-18 18:00:00	4	4	f	\N
247	2025-12-19 18:40:47	2025-12-24 23:04:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	247	Fish Are Friends, Not Furniture	A beginner-friendly foray into the world of fishkeeping, aquascaping and the wonderful world of aquariums	2026-01-18 16:00:00	2026-01-18 16:45:00	2	2	f	\N
190	2025-12-19 16:34:11	2025-12-24 22:42:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	190	Boston Gaymers Game-Show Hour	Do you like video games? Do you know about video games? Do you like small silly prizes? Come join us for a some light hearted fun with several trivia based challenges	2026-01-17 23:00:00	2026-01-18 00:30:00	12	7	f	\N
168	2025-12-19 16:31:02	2025-12-24 22:40:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	168	Furry Logic's Escape Room Time Slot 9	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 20:00:00	2026-01-17 21:30:00	16	7	f	\N
70	2025-12-19 16:15:55	2025-12-24 22:01:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	70	Quantum Fuzzics 101	A smol wolf infodumping about smol physics :3. Come join me in pawstulating the secrets of nature, from Schrodinger's cat to gauge theory. This panel will be a high-level overview of the intuition, experiments, and math behind quantum mechanics, cast in a silly furry lens. It is recommended that attendees be at minimum comfortable with basic algebra (bonus points for linear algebra). 	2026-01-16 23:00:00	2026-01-17 00:15:00	10	2	f	\N
106	2025-12-19 16:21:20	2025-12-24 22:47:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	106	BARQ!Cast LIVE	We're the official podcast of BARQ! Whether you bark, bah, yip, yell, or cluck, this podcast is something you won’t wanna miss! Listen to Banjo & Harbee yap about whatever is on their mind alongside our SECRET special guest...	2026-01-17 04:30:00	2026-01-17 05:15:00	8	10	f	\N
264	2025-12-19 18:43:00	2025-12-24 23:02:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	264	Noisemaker Showcase Meetup	Fans of squeaking, honking, whirring, beeping, and all other playful sounds delight! Now's your chance to show off your favorite ways to make some noise. Show how you've incorporated noisemakers into your fursuits, toys, props, or see how others have done so themselves! Even if you don't have your own sound, fear not, as free squeakers and groan tubes will be provided at the event!	2026-01-18 18:00:00	2026-01-18 18:45:00	11	8	f	\N
282	2025-12-19 18:45:00	2025-12-24 22:35:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	282	Furry Bingo	SECOND ANNUAL FURRY BINGO!! BIGGER CAPACITY BUT SPACE IS LIMITED!! COME EARLY!! AWESOME PRIZES!! AWOOO!!!	2026-01-18 20:30:00	2026-01-18 21:15:00	3	7	f	\N
37	2025-12-19 16:06:48	2025-12-24 21:55:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	37	TRADING CARD DESIGN! History, Function, and Maybe You Can Too?	I LOVE TRADING CARDS AND I WANT TO TELL YOU ABOUT THEM! Trading cards are fun collectables that have a lot of versatility in their function. They can act as art pieces, game pieces, and in many cases both. But, what if you wanted to make your own trading cards? Well, I have! Lets talk about card design, function, and how to make your own cards or even card games!	2026-01-16 19:00:00	2026-01-16 20:00:00	2	7	f	\N
279	2025-12-19 18:44:43	2025-12-24 22:35:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	279	4546B Biology	Get ready to dive in to the speculative biology of the Subnautica game series with Fedora Fennec! This eductaional panel will take you from the safe shallows to sector zero to learn about the flora and fauna that inhabit planet 4546B. It will look at the ecology, conservation, evolution, and even explore real life connections to life here on earth.	2026-01-18 20:00:00	2026-01-18 21:15:00	8	7	f	\N
272	2025-12-19 18:43:54	2025-12-24 22:36:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	272	So You Want to Be a Conductor?	When we watch an orchestra, nearly everyone sees the conductor waving his or her arms at the front of the ensemble and says "What exactly are they doing?" Well, come find out what goes into conducting, and learn some of the basics yourself! Capriccio Meinl teaches at several music colleges and conservatories in the Boston area, composes new music, and conducts several ensembles, including the oldest continuously operating musical society in the US recognized by the Guiness Book of World Records. He will teach you some of the basics of conducting. This panel includes learning basic conducting patterns, left-hand expression technique, score preparation, and conducting to recording.	2026-01-18 18:30:00	2026-01-18 20:15:00	9	11	f	\N
222	2025-12-19 18:36:58	2025-12-24 22:19:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	222	Creepy Critters - Fans, Creatures, and Tropes of the Horror Genre	For fans of Horror to talk about tropes within the genre as well as various creepy critters that may relate to the furry fandom or how their own sonas may add to that.	2026-01-18 03:30:00	2026-01-18 04:45:00	3	2	f	\N
104	2025-12-19 16:20:25	2025-12-24 22:47:08	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	104	Little Critter Classroom	Ever wanted to go back to elementary school and feel like a kid again? Little Critter Classroom is a place for babyfurs, kidfurs, and those curious to experience a fun immersive classroom environment. With lessons, storytime, and even a rainbow parachute! Come, make friends, play, and be a little critter if you want. 	2026-01-17 04:00:00	2026-01-17 05:45:00	6	12	f	\N
253	2025-12-19 18:41:41	2025-12-24 23:04:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	253	Making ASMR (or: How I Learned to Stop Worrying and Love Playing Two Characters at Once)	Do you want to learn how to make ASMR?  Are you curious about the acting methods used to make a roleplay feel immersive and engaging, or maybe even intrigued in participating in a demonstration?  Perhaps some burning questions about surface level audio and video production?  All that and not much else at this panel run by one of the furry ASMR channels ever!	2026-01-18 16:00:00	2026-01-18 05:15:00	6	2	f	\N
38	2025-12-19 16:10:49	2025-12-24 21:55:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	38	Advice from an Amateur Artist: How I Taught Myself to Draw	An intro to drawing fundamentals to form a path towards your art journey. Learn about free beginner practices, teachers, and resources available for self taught artists. From lines to anatomy, come see what you can apply to your doodles to brighten someone’s day!	2026-01-16 20:00:00	2026-01-16 20:45:00	3	3	f	\N
65	2025-12-19 16:14:29	2025-12-24 22:00:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	65	Level Up Your Game! Guide for Running RPGs	For new and veteran players and GM's alike, learn how you can better run a role playing game. Keep your sessions on track with out railroading. Keep players engaged and coming back for more! 	2026-01-16 22:30:00	2026-01-16 23:45:00	8	7	f	\N
261	2025-12-19 18:42:38	2025-12-24 23:03:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	261	Yu-Gi-Oh TCG Meetup!	It's time to Duel!! Let's rev it up with the super special awesome children's card game Yugioh! Whether you're a competitor player ready to get your game on, or a new player wanting to feel the flow, this event is open for everyone to share their love for this tcg we hold dear! Bring your binders, your decks, and Kuriboh plushies to come hang out and celebrate the card game worthy of Ancient Egyptian Pharoahs! (PLS keep evil possessive spirits and cursed milennial items at the door pls!!) 	2026-01-18 17:30:00	2026-01-18 18:15:00	9	7	f	\N
295	2025-12-19 18:46:39	2025-12-24 22:33:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	295	Barbershop (Quartet) Harmony 101	This is not about getting your hair did. I welcome you to the wonderful world of barbershop music: an A Capella art form made up of four voices in close harmony in pleasing consonance. Barbershop music practice is alive and thriving today with an emphasis on perfect tuning, audio physicality, performance, and more! We will go over the basics of barbershop music, example songs, theory, culture, and maybe you too can learn a very short song, also known as a tag. This panel will be part lecture, part musical listening, and part participation. You are not required to be a musician or a singer to attend, but there will be opportunities to participate in a low-stakes environment!	2026-01-18 22:00:00	2026-01-18 22:45:00	11	2	f	\N
145	2025-12-19 16:27:24	2025-12-24 22:38:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	145	Big Cat Meet!	My panel will be for big cat sonas to meet up together and embrace our sona species!	2026-01-17 18:00:00	2026-01-17 18:45:00	7	8	f	\N
82	2025-12-19 16:17:43	2025-12-24 22:46:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	82	Adult Therian Meetup (21+)	Come meet other adult non-humans and chat about what it's like being a *insert creature here* who has to file taxes. Discussion style setup for first half and free-form socializing for the second half!	2026-01-17 00:30:00	2026-01-17 01:45:00	6	8	f	\N
103	2025-12-19 16:20:17	2025-12-24 22:47:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	103	Rito's Rowdy Raccoon Saloon	Hey howdy, Rito here. I wanna get rowdy. Let's get rowdy! Screen games, improv. Really gonna be an off the cuff romp where crowd interaction drives the experience. 1st edition. Scuffed nightmare. Yeehaw. We will yeehaw. We will hoot. AND THERE WILL BE NO QUESTION ABOUT IT. WE WILL HOLLER.	2026-01-17 04:00:00	2026-01-17 05:15:00	2	10	f	\N
259	2025-12-19 18:42:22	2025-12-24 23:03:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	259	Photo Room	Join us in the Photo Room for an extremely premium photo experience, completely free of charge! All attendees are welcome. We offer digital files and physical prints. (Please visit the ANE website for more details.)	2026-01-18 17:00:00	2026-01-18 23:00:00	1	1	f	\N
10	2025-12-19 08:32:54	2025-12-24 08:27:19	us1wm95uilu3p7mh	us1wm95uilu3p7mh	10	So you want a fursuit?	Now what? We’ll discuss the ins and outs of finding the maker or premade that catches your little heart. How go about speaking to a maker and other things that are often missed in other panels.	2026-01-16 14:30:00	2026-01-16 15:45:00	2	2	f	\N
96	2025-12-19 16:19:31	2025-12-24 22:46:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	96	AD: WAM Meet and Greet	A Meet and Greet for furs into Wet and Messy fetish (WAM), i.e. slime, pies, mud, and similar messy things.	2026-01-17 03:00:00	2026-01-17 03:45:00	6	12	f	\N
300	2025-12-19 18:47:14	2025-12-22 14:23:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	300	Sunday Night Dances: The Dead Dog	**07:30 PM** Laelia - _Hyperpop, Club_<br>**08:30 PM** Sneky The Junglist - _Hardgroove Techno_<br>**09:30 PM** Mere Notilde - _House, Future Funk, UKG1_<br>**10:30 PM** Sunsprite - _Makina/Hardtrance (HAPPY HARDCORE?)_<br>**11:30 PM** Pfil Zone _Freetekno Tribe / TRIBAL HOUSE & HARDCORE_	2026-01-19 00:30:00	2026-01-19 05:00:00	10	6	f	\N
219	2025-12-19 18:34:43	2025-12-24 22:44:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	219	Object Show Fans Meet-Up!	Do you love object shows and wish you could meet other fans? Never heard of the Object Show Community and wanna learn more? (OSC) Come yap about your favorite Object Shows, comics, camps, and Object-sonas/OCs! 	2026-01-18 03:30:00	2026-01-18 04:15:00	11	8	f	\N
291	2025-12-19 18:45:55	2025-12-24 22:33:31	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	291	Puppet Ninja Warrior	Are you ready for an extreme obstacle course full of jumps, drops, climbing, and creative solutions? For PUPPETS? Join us for a team-based puppetry activity where you and 2-3 pals pilot a puppet through a precarious miniature obstacle course! You’ll learn how to work as a team to get your puppet through the pitfalls and emerge triumphant! Bring a few pals or get ready to team up with new friends!	2026-01-18 21:30:00	2026-01-18 22:45:00	6	3	f	\N
102	2025-12-19 16:20:07	2025-12-24 22:03:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	102	Character Appeal in Animation	Are you a furry animator struggling to make your characters pop? Join longtime animator Zera in an informative and educational panel designed to help you give your creations the appeal they deserve! Whether you're a newcomer or a seasoned veteran, we'll discuss some nifty tips and tricks to help polish your work and elevate it to the next level, as well as what it's like working in a turbulent yet fascinating industry.	2026-01-17 04:00:00	2026-01-17 04:45:00	7	2	f	\N
26	2025-12-19 08:54:37	2025-12-24 08:31:11	us1wm95uilu3p7mh	us1wm95uilu3p7mh	26	Pokemon VGC & TCG Tournament	Attention Pokemon trainers! Get ready for an all-out battle using the current VGC or TCG format! Bring your own Switch systems or deck and compete for a chance to take home a prize! Pre-register in the video game and board game room, as there is limited space available.	2026-01-16 16:00:00	2026-01-16 19:00:00	12	7	f	\N
232	2025-12-19 18:38:35	2025-12-24 22:19:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	232	Untitled Panel	Insert the idea of this panel in your mind or just come find out what this panel is about?<br>_<br>yes this is a real panel_	2026-01-18 06:30:00	2026-01-18 07:15:00	6	3	f	\N
268	2025-12-19 18:43:25	2025-12-24 23:02:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	268	Weird Old Laserdisc	Like laserdiscs? Ever wanna watch a mechanic training video from the 70's? A airport ticket kiosk? Instructional video on Japanese knee surgeries? This is your panel!	2026-01-18 18:00:00	2026-01-18 19:45:00	8	2	f	\N
274	2025-12-19 18:44:07	2025-12-24 22:36:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	274	Art Doll Creation	An introduction on how to create art dolls. I will use examples from books and my own process. Bring your own dolls/creations to show off if you have them! 	2026-01-18 19:00:00	2026-01-18 20:15:00	6	3	f	\N
257	2025-12-19 18:42:09	2025-12-24 22:20:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	257	Veterinary Furs Meet-Up	Are you a furry that works in or is in school for the veterinary field or similar field of biology, zoology, or medicine? If yes, then this meet-up is for you! Hosted by Fedora Fennec a licenced veterniary technician, invertebrate zoologist, and conservation resercher come meet others in similar fields to share experiences and resources!	2026-01-18 17:00:00	2026-01-18 17:45:00	8	8	f	\N
181	2025-12-19 16:32:43	2025-12-24 22:42:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	181	Stupendium & Friendiums Nerdcore Q&A!	Got a question about content creation? Looking for song writing tips? Have a whimsical enquiry about favourite pasta shape? Join The Stupendium and their musical friendiums for an hour of hopefully informative or at the very least entertaining banter with them and other musical guests!	2026-01-17 22:00:00	2026-01-17 22:45:00	10	2	f	\N
235	2025-12-19 18:39:01	2025-12-24 23:06:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	235	Classic SpongeBob Trivia with SpongePup!	Are you ready to be challenged on your knowledge of the first three SpongeBob seasons? Come join the porous puppy himself! Take a seat, have fun, and most importantly, win kandi prizes! Or maybe the most important part is the fun...	2026-01-18 13:30:00	2026-01-18 14:15:00	2	10	f	\N
88	2025-12-19 16:18:30	2025-12-24 22:46:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	88	Majik Mystery Loot Brawl (18+)	$40 entry fee.Max 30 players.Every player wins.Yankee Gift Swap FOR ADULTS? Buy-in for this fast-paced game of theft & blind luck. Toys, tails, art & more! You just need to choose wisely & hope no one steals your prize. Yes, STEAL! Every choice is a gamble! Never fear, no one leaves empty handed. (Many items include adult themes, including nudity, and other NSFW things.)	2026-01-17 01:30:00	2026-01-17 03:15:00	13	7	f	\N
92	2025-12-19 16:18:58	2025-12-24 22:46:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	92	Hypnokink 101 (18+)	Think your sona is the only one who can be hypnotized? Come learn the 101s of hypnokink for both hypnotists and subjects. The goal is to get you the resources to know where to safely and consentually start your hypnokink journey and the basics of doing hypno in the bedroom from both sides of the pocket watch. This is an entry level class so no need to know anything about hypnosis or kink to come and learn.	2026-01-17 02:30:00	2026-01-17 03:45:00	7	2	f	\N
265	2025-12-19 18:43:05	2025-12-24 23:02:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	265	Alternative Music History	For this panel I'd like to talk about alternative music history. Share a brief history on it then if there is still time have everyone talk to each other about their favorite music/bands. I'd like this panel be a safe space for all people to come together to talk about their favorite bands/artists	2026-01-18 18:00:00	2026-01-18 18:45:00	3	11	f	\N
244	2025-12-19 18:40:19	2025-12-24 23:05:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	244	Gramophones Galore: A Crash Course on Really Antique Devices	Remember those funny-looking boxes with a needle and a horn? Did you know they make noise? And MUSIC??Introducing Gramophones! An audio-playback technology that revolutionized music from the 1880s to the 1950s. Learn about their history, the mechanics, and get hands-on with gramophone artifacts over 100 years old!We're also playing a real life gramophone, including a Fallout track in original format!Neat, huh?	2026-01-18 15:00:00	2026-01-18 16:15:00	3	2	f	\N
7	2025-12-19 01:41:19	2025-12-24 08:25:07	us1wm95uilu3p7mh	us1wm95uilu3p7mh	7	Thursday Night Dances: The Kickoff	- **09:00 PM** - **YatchiDisco** - _Funky House_\n- **10:00 PM** - **Billboard!** - _Drum and Bass_\n- **11:00 PM** - **Spikeo** - _House, Bass House, Funk House_\n- **12:00 AM** - **SkeptrixMelodic** -- _dubstep, Pop and bass house_ \n- **01:00 AM** - **Stampede!** (Envy & Kanon) - _Bass Music_	2026-01-16 02:00:00	2026-01-16 07:00:00	5	6	f	\N
233	2025-12-19 18:38:40	2025-12-24 23:06:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	233	Furry Finance 101: An Actuary's Guide to Retirement	What do fursuits, conventions, and retirement have in common? Math! Join a furry retirement actuarial analyst and explore the world of probability, long term planning, and how budgeting for conventions and fursuits isn't that much different from saving for retirement. Expect fun graphs, laughs, and maybe a minor existential crisis. (No math required)	2026-01-18 13:00:00	2026-01-18 14:15:00	6	2	f	\N
260	2025-12-19 18:42:26	2025-12-24 23:03:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	260	Salut! Discussing the Lore of the Little Tail Bronx Universe!	Join me in semi open-ended discussion of the universe of CyberConnect2's Little Tail Bronx games and adjacent media, including Tail Concerto, Solatorobo, and Fuga: Melodies of Steel.  Talking points include evolution of the gameplay across the series, summarizing and linking together storyline elements that build the overall timeline (while trying to avoid specific game spoilers), and CyberConnect2's connection to the kemono community.	2026-01-18 17:30:00	2026-01-18 18:15:00	13	7	f	\N
236	2025-12-19 18:39:11	2025-12-24 23:06:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	236	Build a Badge Workshop	Whether you're a first time fur, a convention regular, or want a cool memento of your time at ANE, badges are a great way to show off your fursona and introduce yourself to new friends! Stop by and make a goofy googly-eyed badge for yourself (for free) with some crafty critters! We'll bring the supplies and the laminators, you supply the creativity! No art skills required!	2026-01-18 13:30:00	2026-01-18 15:15:00	11	3	f	\N
243	2025-12-19 18:39:59	2025-12-24 23:05:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	243	Bird Roundub: Rancher's Style! (Pájaro Ranchero)	ARE YOU A BIRD?!  Regardless of your answer, you should check out ANE 2026's Bird Roundup! Come squak and enjoy the vibes, and enjoy bird behavior LIVE!	2026-01-18 15:00:00	2026-01-18 15:45:00	5	8	f	\N
256	2025-12-19 18:42:06	2025-12-24 23:03:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	256	Furby Meetup	Do you like those silly Furby toys? Come meet and hang out with other Furby fans! Show off your Furby friends!! Furby-sized party hats will be supplied 	2026-01-18 17:00:00	2026-01-18 17:45:00	11	8	f	\N
9	2025-12-19 08:28:21	2025-12-24 08:25:07	us1wm95uilu3p7mh	us1wm95uilu3p7mh	9	Registration (Friday)	\N	2026-01-16 14:00:00	2025-12-20 01:00:00	4	4	f	\N
13	2025-12-19 08:37:35	2025-12-24 08:28:29	us1wm95uilu3p7mh	us1wm95uilu3p7mh	13	Execute the Clown - A brief Introduction to my favorite sci-fi furry roleplay game, Space Station 14	One of my favorite games and nobody has ever heard of it. Until now.In addition to explaining what this game is all about, I will also have various stories from the game such as:The multiple instances where medical staff have argued that lobotamies are ethical when the patient consentsThe time a mouse sued the station for using mousetrapsThe time a charger murdered multiple peopleAnd more	2026-01-16 15:00:00	2026-01-16 15:45:00	9	7	f	\N
12	2025-12-19 08:36:03	2025-12-24 08:27:59	us1wm95uilu3p7mh	us1wm95uilu3p7mh	12	Wildfire Yoga - Hip and Knee Health	Join me, Awe Tiger, for a 3 part journey into yoga, meditation, and functional movement theory.  Each day will have a different focus - Day 1: Hip and Knee Health.  Day 2: Shoulders, Spine and Breath.  Day 3: Paws and Purrception - Quadrobics Conditioning.  Come explore these traditional health and spirituality based eastern practices meant to free one to the possibility of living in alignment with nature and unleashing the power of their animal body.  All classes will offer options for those with limited mobility who wish to practice from a chair.	2026-01-16 14:30:00	2026-01-16 16:15:00	8	2	f	\N
24	2025-12-19 08:52:03	2025-12-24 08:30:38	us1wm95uilu3p7mh	us1wm95uilu3p7mh	24	Canine Meet & Greet!	Whether your a domestic dog, a wild dog, or a pre-historic dire-wolf. Come meet others in your family! We will be handing out some coloring pages, art supplies, and we will have a fursuit photo at the end!!	2026-01-16 16:00:00	2026-01-16 16:45:00	11	8	f	\N
15	2025-12-19 08:40:50	2025-12-24 08:28:58	us1wm95uilu3p7mh	us1wm95uilu3p7mh	15	Beastars` 10th anniversary celebration meetup!	2026 marks the 10th anniversary of Beastars! Ever since its first chapter was published in 2016, Legosi's story has left a legacy not only in anime and manga, but in the furry community. To kick off its 10 year milestone, let's meet up and discuss its highlighting moments, its identity in manga/furry spaces, and what this story means to us. Let\\`s get walking on the wild side and wish a happy anniversary to Beastars!!	2026-01-16 15:00:00	2026-01-16 15:45:00	11	8	f	\N
22	2025-12-19 08:49:25	2025-12-24 08:30:12	us1wm95uilu3p7mh	us1wm95uilu3p7mh	22	First Furry Convention	Is this your first furry Convention? Come learn the ropes with the ANE staff team. We'll be going over how to tackle such a daunting feat for the weekend. We'll be covering everything from common questions to safety and everything in between. There will be a couple staffers in suits for a great in person examples. There will also be a Q&A after the presenting part.	2026-01-16 16:00:00	2026-01-16 16:45:00	10	2	f	\N
20	2025-12-19 08:47:53	2025-12-24 08:29:57	us1wm95uilu3p7mh	us1wm95uilu3p7mh	20	Crazy Travel Stories	Share your crazy, funny, or insaine stories from your travels.	2026-01-16 15:30:00	2026-01-16 16:15:00	3	10	f	
3	2025-12-19 01:33:17	2025-12-24 08:26:14	us1wm95uilu3p7mh	us1wm95uilu3p7mh	3	Armor Academy: Plate Armor	In this panel, we will teach you how to make your own plate armor, and leave you with a bracer (forearm guard) literally forged by your own hands. There will be a 30 dollar materials fee to participate, but any may observe for free.	2026-01-16 00:00:00	2026-01-16 01:15:00	3	3	f	\N
89	2025-12-19 16:18:39	2025-12-24 22:03:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	89	Zoids Battlegrounds	The giant animal mecha miniatures card game returns again to ANE!Learn to play, dominate the battlefield!Join our Telegram chat: ZoidsFans	2026-01-17 02:00:00	2026-01-17 03:30:00	12	7	f	\N
187	2025-12-19 16:33:35	2025-12-24 22:17:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	187	Furry from the Deep: A Look at the Many Anthro Aliens of Doctor Who	In a Universe as large as this one, you're bound to run into a few furs! Join us as we take a retrospective look back on 60 years of anthro aliens, monsters, and mutants. From bats to moths, lizards to rhinos - not to mention the many, MANY distinct species of cat-people. Whether your favorite Doctor is David Tennant or Patrick Troughton, I'd love if you'd join me on this adventure through time and space!	2026-01-17 23:00:00	2026-01-17 23:45:00	8	10	f	\N
193	2025-12-19 16:34:33	2025-12-24 22:43:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	193	VR Meetup	Meet up with fellow Virtual Reality enthusiasts in real life, no headsets required! Whether you're a VRChat veteran, or merely curious about the hobby, all are welcome!	2026-01-17 23:30:00	2026-01-18 00:15:00	10	8	f	\N
98	2025-12-19 16:19:41	2025-12-24 22:46:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	98	Outer Limits Photo Zone	It’s always after dark in The Outer Limits. Put on your most alluring outfit and weave your way through the crowds. Everyone turns their head as you breeze past: animals, humans, and so many mysterious creatures. Set foot on the red carpet and strike your most seductive poses. It's time to steal hearts and own the night! 🤩	2026-01-17 03:00:00	2026-01-17 06:00:00	10	13	f	\N
196	2025-12-19 16:34:51	2025-12-24 22:43:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	196	A Crash Course in the Horrors of Live2D	So you want to become a VTuber, huh? But you don't want to commission one, and Live2D is really scary to look at? Well Befish is here to be your tour guide in the land of Live2D! Befish will walk you through all of the ins and outs of the program so you're ready to rig your very first model! 	2026-01-18 00:00:00	2026-01-18 01:15:00	8	2	f	\N
225	2025-12-19 18:37:32	2025-12-24 22:49:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	225	Photo Room: Rubber Animals (18+)	Join us for a special after-hours opening of the Photo Room designed for extremely premium rubber photos! We'll have experts available to help shine your rubber gear and make you brighter than the sun. 18+ only. ID is required. (Please visit the ANE website for more details.)	2026-01-18 05:00:00	2026-01-18 06:00:00	1	1	f	\N
90	2025-12-19 16:18:43	2025-12-24 22:46:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	90	The Kandi Pawty! (18+)	Are you a kandi kid? A raver? Or maybe you just want to make some of those super cool bracelets? Then this panel is for you! We will provide everything needed to make (ONE) Free bracelet per person, as well as a free to enter raffle! PLURR	2026-01-17 02:00:00	2026-01-17 03:45:00	3	3	f	\N
255	2025-12-19 18:42:00	2025-12-24 23:03:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	255	Bee-Lieve it or Not!	BEES!!! Ah yes the BEES! Massachusetts is home many fabulous native bee species. These lovely critters often get overlooked in favor of their charismatic cousins the honeybee. So come and buzz on in and hear of the wonderful and sometimes bizarre things our bees do be doing!	2026-01-18 17:00:00	2026-01-18 17:45:00	2	2	f	\N
280	2025-12-19 18:44:51	2025-12-24 22:35:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	280	Furry Logic's Escape Room Time Slot 15	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website <https://www.bookeo.com/furrylogicllc> to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at [furrylogicllc@gmail.com](mailto:furrylogicllc@gmail.com)	2026-01-18 20:00:00	2026-01-18 21:30:00	16	7	f	\N
263	2025-12-19 18:42:50	2025-12-22 14:20:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	263	Stupendium & Friendiums Furry Nerdcore Showcase	Join the Stupendium and their musical friendiums for a showcase of some of the top talent where furry and nerdcore collide. Furrdcore? Nerdpaws? Difficult to explain to the IRS? Call it what you like but one thing is for certain, it’s a show you won’t want to miss!<br><br>Hosted by The Stupendium and also featuring performances from CK9C, Shwabadi, Silva Hound, Ivycomb and Freeced, it’s a musical menagerie almost worth not sleeping in on a Sunday for!	2026-01-18 17:30:00	2026-01-18 19:30:00	15	9	f	\N
93	2025-12-19 16:19:05	2025-12-24 22:46:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	93	Petplay 101	Are you curious about those folks wearing hoods and collars? What or who even are they? AND WHERE DO I SIGN UP?This panel will be a introduction to all things petplay, presented by Pup Crackerjack, an organizer of events in New England. All critters, handlers, and our curious friends are encouraged to join us for a tail wagging good time!	2026-01-17 02:30:00	2026-01-17 03:45:00	2	2	f	\N
120	2025-12-19 16:23:20	2025-12-24 22:04:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	120	Warrior Cats Animation Round Up	For many furries, the Warrior Cats book series by Erin Hunter were their introduction into fursonas, art, animation, and making OCs and the series continues to be a source of inspiration for hundreds of artists today as the series has progressed into 100+ books. Come join Cloverkit and Badgerbuck for a screening of our favorite and most iconic fandom animations while chatting with other fight cat fans.	2026-01-17 14:30:00	2026-01-17 15:15:00	2	3	f	\N
301	2025-12-19 22:30:54	2025-12-22 14:24:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	301	Spooky Creature Feature	A meet and greet for Skulldogs, Cryptids, Hellhounds, Demons, Monsters and all other kinds of spooky creatures. Come and meet your fellow scary creatures during the coldest time of year. A group photo will be taken at the end of the panel.	2026-01-18 02:00:00	2026-01-18 02:45:00	13	8	f	\N
5	2025-12-19 01:37:42	2025-12-24 08:26:31	us1wm95uilu3p7mh	us1wm95uilu3p7mh	5	Prosthetic FX Makeup Demo	Learn how to transform people into animals using foam latex prosthetic muzzles and special effects makeup. This panel will consist of a live demo application from start to finish, and will cover topics such as materials, techniques, and pitfalls, as well as plenty of opportunities to ask questions!	2026-01-16 01:00:00	2026-01-16 02:45:00	2	5	f	\N
127	2025-12-19 16:24:09	2025-12-24 22:04:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	127	How to REALLY Not Get Con Crud!	With colds and new Covid variants everywhere, plus emerging nasties like H5N1 around the corner, coming home sick after a con might seem inevitable – but what if it doesn’t have to be this way? Learn how to use masks, tests, air filters, and other new tools to create a strategy that protects yourself and others, whether you’re at a party, in fursuit, or dealing in the den. We’ve got FREE N95 masks and fit testing to help you find your perfect match, FREE Covid tests to help keep you and your friends informed, and demos of DIY air filtration technology to help control both respiratory diseases and flying fur! Help us keep furry a safe and accessible space for all.	2026-01-17 15:30:00	2026-01-17 16:45:00	2	2	f	\N
115	2025-12-19 16:22:42	2025-12-24 22:14:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	115	Mascot Meet-up and Q&A	Are you a current or former performer with stories to share? Or just curious about what life is really like backstage? Come and hear us share our thoughts and maybe reveal some secrets.	2026-01-17 13:00:00	2026-01-17 14:15:00	2	5	f	\N
293	2025-12-19 18:46:14	2025-12-24 22:33:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	293	An Introduction to "Content Creation" w/BetaEtaDelota	Have you ever wanted to get into Content Creation through YouTube, Twitch, TikTok, etc. but don't know where to start or what to do? Join BetaEtaDelota for "Introduction to Content Creation", a beginner-friendly panel covering the fundamentals of DOING THE THING! Learn practical tips, creative inspiration, and essential tools to help you confidently share your passion, craft, or ideas online!	2026-01-18 21:30:00	2026-01-18 22:45:00	8	2	f	\N
119	2025-12-19 16:23:13	2025-12-22 14:06:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	119	Registration (Saturday)		2026-01-17 14:00:00	2026-01-17 23:00:00	4	4	f	\N
298	2025-12-19 18:47:04	2025-12-22 14:23:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	298	Charity Auction	Come on down to Main Events to lock in your final bids on those items you've been seeing at the Charity Table in the Vendor Hall!<br><br>The charity auction, hosted by Rocky Glimmer-LaRouge and Bee Bunsen-Buckley, is your last chance to help us raise even more money for our two amazing charities while walking away with all kinds of cool auction items!	2026-01-18 23:00:00	2026-01-19 00:00:00	5	9	f	\N
270	2025-12-19 18:43:40	2025-12-24 22:36:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	270	Suiting with Animal Characteristics	You may have fur suited, but have you been suited in fur? Watch me nerd out about the important fibres animals have contributed to mens fashion in the 20th century with the potential for overly formal hot takes!	2026-01-18 18:30:00	2026-01-18 19:45:00	13	2	f	\N
189	2025-12-19 16:33:59	2025-12-24 22:17:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	189	A Venture to Rock Bottom: Bikini Bottom's Conspiracy Panel	Join expert panelists Solvi and Lilith and uncover the hidden truths the Elites From Under the Sea don't want you to see. A pineapple under the sea? A bubble 'buddy'? Audience members are encouraged to help Drain The Lagoon by contributing their own theories as well.	2026-01-17 23:00:00	2026-01-18 00:15:00	7	10	f	\N
54	2025-12-19 16:13:10	2025-12-24 21:59:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	54	(F)urban Planning	A panel for anyone interested in urban planning! We'll be taking a look at the cityscape around the convention center, and then planning our own improvements using maps, drawings and other tools. This is an interactive panel where all knowledge levels are welcome!	2026-01-16 21:00:00	2026-01-16 22:15:00	3	5	f	\N
87	2025-12-19 16:18:25	2025-12-24 22:03:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	87	Historic Preservation in the Furry Fandom	Cultural groups may not realize the importance of historical preservation until after knowledge and artifacts have been lost or discarded. Come join a discussion about the challenges of preserving furry history, where the fandom stands today and what needs to be done in the future to build the institutions and infrastructure to preserve the past. The goal of this panel is to raise awareness and help those interested start taking actions in their local community.	2026-01-17 01:30:00	2026-01-17 02:45:00	8	2	f	\N
76	2025-12-19 16:16:58	2025-12-22 14:01:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	76	Furtography 102: Where to Get Started	Heard the term "Rule of Thirds"? What about ISO? F-stop? Why even bother with "real" cameras when smartphones come with one already?This panel is a No-Experience-Needed Beginner to Intermediate crash course on the basics of fur-tography, going over terminology, fundamentals, and why it just might be worth it to take that next step beyond the phone camera. More than just telling you information you can find online, I hope to explain just what about photography as a hobby has me so hooked.	2026-01-18 02:30:00	2026-01-18 03:45:00	10	\N	f	\N
180	2025-12-19 16:32:27	2025-12-24 22:42:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	180	How to Handsew 101: Sewing a Tail	Welcome attendees to a panel to learn about the basics of hand sewing fur. Learn what tools are best to use, what supplies you'll need and have the opportunity to create a tail alongside the instructor! You'll be leaving with supplies to help you further down the line as well as knowledge to assist in the next steps of suit making!	2026-01-17 21:30:00	2026-01-17 23:15:00	3	2	f	\N
221	2025-12-19 18:36:44	2025-12-24 22:49:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	221	Bodybuilding 101: Becoming a Furry Bara Beef Boy	Do you like Muscle? Want to learn how to get huge IRL and meet other furs with the same iron bug? Whether you’re a complete beginner or seasoned lifter, this panel has it all: Training, Diet, Supplementation, and Lifestyle. Taught by the Muscle Fursuit Maker himself, Dozer will leverage his 15 years of training experience to deliver the most informative and fun panel of your weekend.  Meet and Greet will followNote: All 18+ Furries are welcome!Note2: This is Bodybuilding Panel, not a Bara Art Panel	2026-01-18 03:30:00	2026-01-18 04:45:00	10	2	f	\N
275	2025-12-19 18:44:13	2025-12-24 22:35:52	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	275	Furry Con Smashbooking + DIY!	Do YOU find yourself coming home from furry conventions with more stickers, pins, badges and other con loot than you know what to do with? Looking to take that old shoebox full of convention paraphernalia and put it to good use? Look no further than Furry Convention Smashbooking! This panel is half presentation and half D.I.Y; tape squares and paper will be provided!	2026-01-18 19:00:00	2026-01-18 20:15:00	3	3	f	\N
179	2025-12-19 16:32:17	2025-12-24 22:41:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	179	Majik Mystery Loot Brawl (ALL AGES)	$30 entry feeMax 30 playersEvery player winsWhite Elephant, Bad Santa, that terrible office exchange where you're the secret Santa of someone you don't know, and there's so much pressure to get the PERFECT gift! Never fear, the pressure is off! We've picked the gifts for you. You just need to choose wisely, and hope no one will steal your perfect prize.Steal? STEAL! That's right, folks, every choice is a gamble! Never fear - No one leaves empty handed. (Maybe something awesome, maybe something silly!) Toys, tees, plush, and art swag! If you are brave, if you're willing to take a risk, c'mon down and fight for your prize.	2026-01-17 21:30:00	2026-01-17 23:15:00	13	7	f	\N
184	2025-12-19 16:33:11	2025-12-24 22:42:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	184	Percussion Jam	Got a lot of excess energy and just want to let it all out? Well now is you chance! come whack some buckets, play a djembe, shake a tambourine, or smack a cajon with other like minded folks and have some fun. This group will be a guided percussion experience with musical games to get folks moving and having some fun. No previous musical experience necessary!	2026-01-17 22:00:00	2026-01-17 22:45:00	7	11	f	\N
169	2025-12-19 16:31:09	2025-12-24 22:16:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	169	The Jerma Appreciation Panel Returns	The Jerma985 panel. Jerma985, or Jerma, is an American live streamer, YouTuber, performance artist, and voice actor known for his elaborate Twitch live streams incorporating surreal comedy.	2026-01-17 20:30:00	2026-01-17 21:45:00	10	10	f	\N
205	2025-12-19 18:32:29	2025-12-24 22:43:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	205	The Last Ride of Roger Web	The famed outlaw, Roger Web, plans to retire, but not until he pulls off one last job.  Join the crew to rob a train as one of several pre-generated characters in this role-playing adventure.  Will you work to expose the master criminal to the authorities or will you maintain honor among this band of rogues?  Each character has thier own secret objective.  Players will be limited.  No experience in any role playing game is required!	2026-01-18 01:00:00	2026-01-18 03:00:00	12	7	f	\N
302	2025-12-19 22:32:23	2025-12-22 14:24:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	302	ANE Found Family: How to Adult	Your ANE Found Family is here to help with all those pesky tasks, questions and skills family is supposed to teach! Sometimes our circumstances leave us wanting answers no one gave us about basic tasks adults are expected to know. Well, we will help you figure out some basic skills like safe networking+community building, how to plan to budget cook things that don't taste terrible, and even maybe how to tie a tie. (I know I still have trouble doing it) Come for the skills, and hopefully leave with new friends (or family!)	2026-01-18 01:30:00	2026-01-18 02:45:00	8	2	f	\N
31	2025-12-19 09:01:57	2025-12-24 08:32:27	us1wm95uilu3p7mh	us1wm95uilu3p7mh	31	Furry Logic's Escape Room Time Slot 1	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website <https://www.bookeo.com/furrylogicllc> to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at [furrylogicllc@gmail.com](mailto:furrylogicllc@gmail.com)	2026-01-16 17:00:00	2026-01-16 18:30:00	16	7	f	\N
35	2025-12-19 09:08:02	2025-12-24 08:33:37	us1wm95uilu3p7mh	us1wm95uilu3p7mh	35	Furry Logic's Escape Room Time Slot 2	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website <https://www.bookeo.com/furrylogicllc> to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at [furrylogicllc@gmail.com](mailto:furrylogicllc@gmail.com)	2026-01-16 18:30:00	2026-01-16 20:00:00	16	7	f	\N
209	2025-12-19 18:33:04	2025-12-24 22:48:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	209	Send in the (Furry) Clowns!	Strap on your red noses and get ready to throw some cream pies, because it's not just rodeo clowns that you can find at ANE this year! Join Blaz the Hyena on a super silly exploration through the world of clowning, from its artistic and comedic aspects to its presence as a kink, and learn about the intersection between clowning and the fandom, a.k.a. clown furs!	2026-01-18 02:00:00	2026-01-18 02:45:00	7	2	f	\N
227	2025-12-19 18:37:49	2025-12-24 22:49:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	227	FenFen's Intro to Rope Play	A hands-on introductory lesson for rope bondage! We will cover safety, scene negotiation, equipment, and teach some fundamental ties so you can get the most out of your next play session. Partners are not required, and (limited) rope will be supplied. Fursuits are welcome, but are not recommended for people doing the tying.	2026-01-18 05:00:00	2026-01-18 06:15:00	2	12	f	\N
228	2025-12-19 18:37:57	2025-12-24 22:57:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	228	Blood on the Clocktower	Blood on the Clocktower is a social deduction game, like Werewolf, Mafia, or Among Us. The town must work together to find the demon before it's too late, while the minions of the forces of evil will lie and misdirect to destroy the town! The game supports up to 20 players, new players welcome! 	2026-01-18 05:00:00	2026-01-18 06:45:00	13	7	f	\N
163	2025-12-19 16:30:20	2025-12-24 22:55:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	163	New York Furs Meet & Greet	All are welcome to join! Come Say hi to some NY fluffs. Some from the far north others from the west, some from Long island we are all over! You could possibly find new locals or make a new friend! 	2026-01-18 17:30:00	2026-01-18 18:15:00	8	8	f	\N
171	2025-12-19 16:31:26	2025-12-24 22:41:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	171	Who is Hatsune Miku?	Taking inspiration from Roflfox's "Who is Hatsune Miku?", this panel will answer questions such as "What the heck is a Vocaloid?", "What is Project Diva?", "What's Miku Expo like?", and most importantly, "Who is Hatsune Miku?"	2026-01-17 20:30:00	2026-01-17 21:45:00	7	2	f	\N
211	2025-12-19 18:33:34	2025-12-24 22:48:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	211	Vore Meetup	Are you a fur with a hungry appetite or an admirer of bellies? Come and meetup some with fellow vore enthusiasts! Make some new friends, or snacks!	2026-01-18 02:00:00	2026-01-18 03:15:00	10	8	f	\N
246	2025-12-19 18:40:41	2025-12-24 23:04:39	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	246	Warriors: The Trivia Battle	War has fallen upon the Four Clans. Join your fellow warrior cats and fight... in a nostalgia triva battle! Deriving from Erin Hunter's Warriors book series, your knowledge will be tested on the early days of the online fandom and its creations from the early 2000s to mid-2010s. Discover which Clan you'll be fighting for, be given your warrior name, and may the best Clan WIN!	2026-01-18 15:30:00	2026-01-18 16:45:00	11	10	f	\N
64	2025-12-19 16:14:20	2025-12-22 13:58:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	64	Pooltoy Meet & Greet	You're invited to hang out and take photos with air-filled critters! We'll have pooltoy experts available to answer all of your questions: how much do they cost, where can I get my own, how do I feed them, ...?<br><br>For your safety and the safety of the critters, please do not bring sharp objects into the event. We'll have an unattended storage area available for you to store your items	2026-01-16 22:00:00	2026-01-17 00:00:00	5	9	f	\N
249	2025-12-19 18:41:04	2025-12-24 23:04:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	249	Dice Goblin Trading Panel	Have a set of dice you regret buying? Swap it! Trade your old sets of dice with your fellow con goers for new (to you) sets for shiny math rocks. And why stop there? Dice trays, character sheets, and other paraphernalia are all fair game.	2026-01-18 16:00:00	2026-01-18 16:45:00	9	8	f	\N
52	2025-12-19 16:12:57	2025-12-24 21:59:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	52	Sonic the Meetup!	We love Sonic the Hedgehog and pals!! This is a place for Sonic fans to meet each other. There will be a silly powerpoint at the beginning to get things started and then the rest of the panel will be social time. Please dress up and bring plushies, merch, or analog games to show off if you want to!	2026-01-16 21:00:00	2026-01-16 21:45:00	7	8	f	\N
78	2025-12-19 16:17:13	2025-12-24 22:02:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	78	Film Scoring in the Fandom	Ever wondered what goes on behind the scenes of the set of your favorite film? Curious to see just who's behind those soundtracks that make you a little misty-eyed as you leave the theatre?All within the confines of New England's most popular con of the year, you'll witness a live scoring session and hosted by two student composers from the Berklee College of Music in Boston. In addition to the real-time demonstration, you'll be introduced a variety of scores, sounds, and talents of an array of brilliant composers in the fandom.	2026-01-17 00:00:00	2026-01-17 01:15:00	11	11	f	\N
132	2025-12-19 16:24:50	2025-12-24 22:05:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	132	Make a Donut in a Blender	Sorry to say, this isn’t your ordinary cooking class! This panel will teach out the basics of working in Blender, a 3D modeling software that has been the major contributor to the rise of custom 3D art across the fandom. Please note: A laptop and mouse is required to join in on the fun!	2026-01-17 16:00:00	2026-01-17 17:15:00	8	2	f	\N
63	2025-12-19 16:14:13	2025-12-24 22:09:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	63	Make Your Own $cale Tail!	Tails can be made of more than fur! Learn the basics of scalemail and use it to make your very own customizable small scaly tail in this paid workshop. Materials will be provided (with color options!) and tools will be lent for the length of the workshop. No prior experience required! 	2026-01-16 22:00:00	2026-01-16 23:45:00	9	5	f	\N
303	2025-12-19 23:02:23	2025-12-24 22:10:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	303	DIY Leathercraft: Leather Collars	In this panel, we will teach you the basics of leatherworking and guide you through making a personalized leather collar. There will be a 25 dollar materials fee to participate, but any may observe for free.	2026-01-17 01:00:00	2026-01-17 02:15:00	9	3	f	\N
113	2025-12-19 16:22:31	2025-12-24 22:03:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	113	City Pop Coffee Shop!	Come start your morning with some chill, nostalgic grooves! Join Icky, fake morning radio DJ and real bird, in an exploration of Japanese city pop, a genre of 80s synthpop that continues to find new fans to this day. This panel is light on lectures: feel free to drop in, bring your own coffee, check your phone, have a chat, and enjoy a gentle start to your day.	2026-01-17 13:00:00	2026-01-17 13:45:00	7	11	f	\N
267	2025-12-19 18:43:17	2025-12-24 23:02:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	267	Meet the Charity: Wings Wildlife Rehabilitation Center	Get up close and personal with the wildlife of New England in this show-and-tell educational panel! Maria and Ark from Wings of the Dawn will be showing off their organization’s work by introducing attendees to ambassador animals and explaining the ins and outs of wildlife rescue work.	2026-01-18 18:00:00	2026-01-18 19:45:00	2	2	f	\N
131	2025-12-19 16:24:43	2025-12-24 22:05:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	131	GoH Meet and Greet	Come hang out with our Guests of Honor for ANE 2026! Join The Stupendium, Rito Bandito, and Sleepy Stag for an exclusive Meet & Greet. After a brief introduction from each of our guests, we’ll be opening the floor for a Q&A session—so bring your best questions! 	2026-01-17 16:00:00	2026-01-17 16:45:00	10	8	f	\N
170	2025-12-19 16:31:17	2025-12-24 22:40:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	170	Six Raccoons in a Trench Coat - Plurality 101	Are you a plural furry? Or maybe you're asking yourself "what the heck is a plural furry???" We're here to answer that and many more plurality questions, and create a safe space for both plural and singlet (non plural) furries alike to learn and socialize! Come learn with us!	2026-01-17 20:30:00	2026-01-17 21:45:00	11	2	f	\N
138	2025-12-19 16:25:28	2025-12-24 22:05:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	138	Creating Character	Develop characters with personality. Writing focused. Bring something to write in if possible, otherwise I will have paper/pens. 	2026-01-17 17:00:00	2026-01-17 18:15:00	6	2	f	\N
139	2025-12-19 16:25:37	2025-12-24 22:05:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	139	So You Wanna Get a Tattoo?	Interested in a tattoo, but don’t know where to start? Panelists Jay (AKA JAYSTOR, a full-time tattoo artist) and Matty (AKA hyenabbq, a former tattoo shop assistant) are sharing advice for first-time tattoo getters! We’ll break down pricing, discuss how tattoos are different from standard art commissions, and define tattoo terms that you may have heard online. Bring your burning questions!	2026-01-17 17:00:00	2026-01-17 18:15:00	2	2	f	\N
166	2025-12-19 16:30:40	2025-12-24 22:16:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	166	So You Want to Make a Visual Novel?	Ever wanted to know what it's like to make a visual novel? Dive into a visual novel development course suitable to both beginners and advanced level for all ages to enjoy! Engage with fellow artists, musicians, developers, writers, and voice actors and learn tips, tricks, and advice. There are infinite possibilities to put your creative ideas expressively through the power of stories and imagination!	2026-01-17 20:00:00	2026-01-17 21:15:00	2	2	f	\N
149	2025-12-19 16:27:45	2025-12-24 22:38:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	149	Hockey Furs Assemble!	Are you a furry? Do you also like hockey?? Well have I got the panel for you! Rep your favorite teams and come on down to hang out with fellow hockey fanatics as the 2026 season is hot underway! Don your favorite jerseys cause we'll be taking a group picture around the middle of the panel! Hope to see all you hockey heads there! Signed, you're favorite Rangers fan lost in Bruins Territory. 	2026-01-17 18:30:00	2026-01-17 19:15:00	13	8	f	\N
158	2025-12-19 16:29:40	2025-12-24 22:39:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	158	J-Pop and K-Pop Random Dance Play!	Do you like dancing to your favorite J-pop and K-pop songs? You should come and join our (incredibly biased) mixed J-pop and K-pop Random Dance Play!How it works: Participants will stand to either side of the room, there will be a countdown, and then 30-45 seconds of a song will play! If you know the choreography, jump into the middle and dance!We will be video recording the random dance play and posting it to YouTube after the con, so keep that in mind when attending.	2026-01-17 19:00:00	2026-01-17 20:15:00	7	11	f	\N
159	2025-12-19 16:29:52	2025-12-24 22:39:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	159	InFURduction to Ballroom	Ever wanted to learn to dance with your friends or partner(s)? Come learn a little of Bachata and East Coast Swing and rock those moves at the dances! No experience or partner required! Wear comfortable shoes. Fursuits welcome but not necessarily encouraged.	2026-01-17 19:00:00	2026-01-17 20:45:00	5	11	f	\N
161	2025-12-19 16:30:05	2025-12-24 22:39:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	161	Majik Mystery PIN BRAWL!	$10/chair30 chairsEvery player wins!The Pin Brawl is a pay-to-play PIN STEALING event, where everybody leaves with a prize! Mostly hard enamel pins, from a wide variety of artists and brands. If you like grab bags and mystery boxes, Pin Brawl is for you! (No advanced purchase, first in line OUTSIDE of the event room will be the first in.)	2026-01-17 19:30:00	2026-01-17 21:15:00	13	7	f	\N
203	2025-12-19 18:32:16	2025-12-24 22:43:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	203	All Ages Sketchbook Swap	Love drawing? Doodling? Want to draw other attendees awesome fursonas and receive art of your own in return?<br>Join us for our all ages sketchbook swap!<br>Bring your own sketchbook, Fursona reference, and drawing materials and get ready to draw!	2026-01-18 00:30:00	2026-01-18 02:15:00	11	3	f	\N
214	2025-12-19 18:34:15	2025-12-24 22:49:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	214	Hypnokink 202 (18+)	Established hypnotist or subject ready to upgrade your skills? Come learn some tips and tricks to deepen your understanding. This panels will go in depth on both hypnotist and subject skills for various topics and will be heavily influenced by audiance request. I am assuming you know hypno and kink basics if you come to this panel.	2026-01-18 03:00:00	2026-01-18 04:15:00	7	12	f	\N
207	2025-12-19 18:32:41	2025-12-24 22:48:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	207	DIY After Dark: Elastic Body Harnesses for the Bedroom (18+)	Want to learn how to make your own spicy-time fun clothes? In this panel, we will teach you how to make elastic lingerie body harnesses, and send you home with one made by your own hands. Tight clothing recommended. There will be a 25 dollar materials fee to participate, but any may observe for free. 18+.	2026-01-18 01:30:00	2026-01-18 02:45:00	3	12	f	\N
43	2025-12-19 16:11:50	2025-12-24 21:58:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	43	Portable PCs from 1986 to 2006	Have you ever seen a glowing gas plasma screen in person? Ever experienced the true compromise of the 90s laptop? Remember your first real laptop that felt like you could leave your desktop behind? I'm here to demonstrate 3 or 4 portable PCs that have been meaningful to me, and have a lot to say about their eras from the 286 up to the Core Duo. 	2026-01-16 20:00:00	2026-01-16 20:45:00	8	5	f	\N
77	2025-12-19 16:17:06	2025-12-24 22:02:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	77	How to Start a Furry Convention	Have you ever wanted to start a furry convention but didn’t know where to begin? Come by this panel and our experienced panelists will discuss the ins, outs, and what-have-yous of starting a con.	2026-01-17 00:00:00	2026-01-17 01:15:00	13	2	f	\N
212	2025-12-19 18:33:40	2025-12-24 22:48:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	212	Photo Room (18+)	Join us for an exclusive after-hours opening of the Photo Room! We welcome your spicy outfits and scandalous poses! 18+ only. ID is required. (Please visit the ANE website for more details.)	2026-01-18 02:00:00	2026-01-18 05:00:00	1	1	f	\N
85	2025-12-19 16:18:10	2025-12-24 22:46:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	85	Adult Survivors of Warrior Cats Childhood	Meet and greet your fellow quiet kids, recess roleplayers, and avid readers to discuss the self-inflicted trauma we all share known as Warriors by Erin Hunter. Share OCs, discuss the insane events of the series, and connect with other adults that still read cat books for middle schoolers. 	2026-01-17 01:00:00	2026-01-17 02:15:00	2	8	f	\N
79	2025-12-19 16:17:20	2025-12-24 22:02:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	79	Pass the Yen Game Show!	Back again for its 8th year at ANE! Calling all fellow furs, come join the fun as we bring to you the excitement of Pass the Yen! Based on a short lived game show on CBS in 1978, this is a quick paced game where contestants battle against each other to see who can survive and win lots of great prizes in this last person (or last furry!) surviving format!	2026-01-17 00:00:00	2026-01-17 01:30:00	12	7	f	\N
40	2025-12-19 16:11:07	2025-12-24 22:12:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	40	Oral Health with Nisha - The Tooth, the Whole Tooth, and Nothing but the Tooth	Nisha - your favorite friendly TUwUth Faerie - is bringing their BlueSky series to Boston! This is a great chance to ask all those dental questions you never get a chance to ask your own dentist. Topics could be squicky and pictures may be pulled up, viewer discretion is advised (let's face it - dentistry is messy)	2026-01-16 20:00:00	2026-01-16 20:45:00	2	2	f	\N
216	2025-12-19 18:34:25	2025-12-24 22:49:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	216	Pony Play	What is Pony Play? What does it take to be a Pony? Is Pony Play only a kink thing? How is Pony Play different than other forms of pet play? How does Pony Play relate to the fandom? and other topics! This panel's goal is to inform people about Pony Play, give an overview, and allow others to ask questions.First half will be a intro class. Secound half will be a mix of stations to see/try different aspects of pony play inclueding carts, pony masks, harnesses, and more!	2026-01-18 03:00:00	2026-01-18 04:45:00	6	12	f	\N
210	2025-12-19 18:33:10	2025-12-24 22:18:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	210	RubberFurs Care 'n Share	We aim to have a two part presentation - we'll begin with a 20 minute presentation about what latex is, how to care for it, and how it interacts with other materials commonly used by furs in their outfits and designs. Afterwords, we will transition to a social where rubberfurs can meet each other, introduce themselves, and mingle. We're hoping this can be both a way for people looking to dip their toes into latex, and a way for people already into Latex to connect.	2026-01-18 02:00:00	2026-01-18 02:45:00	2	2	f	\N
206	2025-12-19 18:32:36	2025-12-24 22:48:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	206	From the Keyboard to Kink Reality	Ready to turn your fantasies into real-world experiences? This panel is designed for those new to kink who want to explore safely, confidently, and authentically. We’ll dive into the essentials. Consent, communication, and negotiation; while also covering how to meet new people, discover roles, and set expectations. You’ll leave with practical tools, safety tips, and a deeper understanding of what makes kink both exciting and empowering. Whether you’re just starting out or curious about taking the next step. This session will help guide your journey.	2026-01-18 01:30:00	2026-01-18 02:45:00	6	2	f	\N
226	2025-12-19 18:37:39	2025-12-24 22:49:39	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	226	Beauty of the Beast: Romanticizing Terrors Among Other Interests	From prehistory to the modern day, people have had a romantic fascination with monsters. This 18+ panel is a deep dive into the subject of teratophilia aka sexual attraction to monsters, showing examples from history and modern pop culture. 	2026-01-18 05:00:00	2026-01-18 06:15:00	6	12	f	\N
39	2025-12-19 16:11:01	2025-12-24 21:58:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	39	Fursuit Quick Draw	Think you are the fastest paw in the east? Come find out at fursuit quick draw! For a $1 donation to charity [Optional], we will holster you up and at ten paces draw your nerf gun and fire! The winner will recieve a gunslinger badge and title of fastest paw in the east!	2026-01-16 20:00:00	2026-01-16 20:45:00	10	7	f	\N
230	2025-12-19 18:38:07	2025-12-24 22:49:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	230	The Making of Yiff! Magazine, a Self-Published Funny Animal Periodical	"Why in the world would you be printing porn on paper in this age? That's why we've got Google!!"Yiff! Magazine is a funny animal periodical that first flew off the presses in 2024. Join us, some of the artists and editors on the project, as we take you through the process of making a self-published bundle of smut. Along the way, you'll learn about the project's inception, some of the diverse array of material we host, and why in the world we're doing this instead of just posting our porn on the computer.And for fans of the magazine, stop on by to see what 2026 may have in store for us. Safe Yiffing!	2026-01-18 05:30:00	2026-01-18 06:45:00	7	12	f	\N
48	2025-12-19 16:12:28	2025-12-24 21:59:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	48	ANE! How does it run?!	Have you ever wondered what goes on behind the scenes of Anthro New England? Come learn the who, what, when, where and how the convention runs so you can have an awesome Boston-y weekend! (You may even learn more where about Copley went last year 👀🦌⁉️)	2026-01-16 20:30:00	2026-01-16 21:45:00	6	2	f	\N
172	2025-12-19 16:31:33	2025-12-24 22:41:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	172	Protogen and Synth Meet-Up!	These ain't mechanical bulls! Come meet and beep protogens and synths of ANE 2026! Learn more about our suits, and network with fellow protogens/synths. Friendly to all folks so don't be shy and say hi!	2026-01-17 21:00:00	2026-01-17 21:45:00	8	8	f	\N
61	2025-12-19 16:14:02	2025-12-24 22:00:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	61	Pin Trading Post	Have too many pins? Looking to spice up your pin collection with something new? Come on over and trade pins with us! Bring pins to trade with other attendees and add some new flair to your collection!	2026-01-16 22:00:00	2026-01-16 22:45:00	11	8	f	\N
66	2025-12-19 16:14:40	2025-12-24 22:01:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	66	The Relaxing J-Pop Oldies' Social Hour	This is a simple panel showcasing J-Pop music/music videos from the late 20th century, featuring a mixture of tunes from the 1960's through the 1990's, as well as discussing how music like this can be legally purchased today. 	2026-01-16 22:30:00	2026-01-16 23:45:00	3	11	f	\N
176	2025-12-19 16:32:03	2025-12-24 22:41:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	176	Cuthroat Artist Gameshow	Watch four contestants bid and battle it out in a series of incredible art challenges! Based on the hit TV series "Cutthroat Kitchen," the artists will be drawing YOUR fursonas under strange conditions, given ridiculous prompts, and their fate will be up to YOU to decide!	2026-01-17 21:30:00	2026-01-17 23:00:00	12	10	f	\N
69	2025-12-19 16:15:04	2025-12-24 22:01:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	69	Pokémon Meet and Greet	Are you a fan of Pokémon? Whether you're just starting out on those grassy routes or have been battling the Elite Four since the beginning, this is the place for you! Come meet, play, trade, and chat with other Pokémon fans! Fursuits are welcome but not required!	2026-01-16 23:00:00	2026-01-16 23:45:00	2	8	f	\N
208	2025-12-19 18:32:57	2025-12-24 22:44:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	208	Puppet Workshop: SNOOFS	THIS PANEL REQUIRES SEWING EXPERIENCE. Join us to sew up some fuzzy hand and rod puppets with tons of personality (and optional arms!)! The Snoof is a fun and free puppet pattern pioneered by Adam Kreutinger. This is an advanced beginners workshop for eager puppeteers looking to make a fuzzy hand puppet of their very own! This panel will have supplies for 20 participants and will require some sewing experience and the ability to work with hot glue guns. All children need to work with an adult guardian, as this puppet requires the use of needles, scissors, and hot glue. Get ready for a lip sync sing along at the end of the workshop to break in those new little critters! 	2026-01-18 01:30:00	2026-01-18 03:15:00	9	3	f	\N
204	2025-12-19 18:32:24	2025-12-24 22:18:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	204	Transformation Meet and Greet	A meet and greet for everyone with an interest in transformation. We would be in a large group to talk about the theme of transformation and then split off into smaller groups to find others that are interested in the parts that interest you the most. Its an easy way to find others who like the same parts of transformation based content and media and a place to chat about what makes it special to you.	2026-01-18 01:00:00	2026-01-18 01:45:00	2	8	f	\N
59	2025-12-19 16:13:51	2025-12-24 22:09:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	59	Spicy Stuff with Seppi	A fun and informative panel where the host walks you through the cultural and culinary history of peppers while subjecting himself to an increasingly intense lineup of hot sauces 	2026-01-16 22:00:00	2026-01-16 22:45:00	6	2	f	\N
101	2025-12-19 16:19:59	2025-12-24 22:14:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	101	Werewolf	Come get the werewolves out of your town, or be a lucky one to try and snack on all the villagers.Will be using the Lupus in Tabula varation of the game and card set. Viewers are most welcomed into the room to game watch. 	2026-01-17 03:30:00	2026-01-17 05:15:00	13	7	f	\N
53	2025-12-19 16:13:03	2025-12-24 21:59:39	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	53	Underrated Furry Animated Films	Are you a fan of animated films starring anthropromorphic characters, but want to find films not in the public eye? This panel is just for you. I plan to discuss films from around the globe, with films that released recently to ones decades old. This panel welcomes all animation fans and shows films from rated G to PG-13.	2026-01-16 21:00:00	2026-01-16 22:15:00	8	10	f	\N
151	2025-12-19 16:27:59	2025-12-24 22:15:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	151	Cowboys! And How They Were Often Secretly Fond of Each Other	Cowboys from the American West to Brokeback Mountain. A brief history of the queer lives of the outsiders whose iconography has come to define the idea of rugged masculinity. A presentation on both cowboys in media and some of the real life examples and inspirations.	2026-01-17 18:30:00	2026-01-17 19:45:00	2	2	f	\N
58	2025-12-19 16:13:40	2025-12-24 22:00:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	58	Hear Us Out - A Selfship and Yumeship Panel!	Have you noticed the rise of yumeshipping and selfshipping? Do you ship your OC, sona or yourself with a character? Here I'll discuss the different eras of selfships. I'll have time at the end for people to talk about their selfships. 	2026-01-16 22:00:00	2026-01-16 22:45:00	13	10	f	\N
142	2025-12-19 16:27:10	2025-12-24 22:37:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	142	Photo Room	Join us in the Photo Room for an extremely premium photo experience, completely free of charge! All attendees are welcome. We offer digital files and physical prints. (Please visit the ANE website for more details.)	2026-01-17 17:00:00	2026-01-18 02:00:00	1	1	f	\N
95	2025-12-19 16:19:22	2025-12-24 22:46:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	95	NSFW Sketchbook Swap (18+)	Want to sketchbook swap with the option to make it ~Spicy~ or draw weiners?<br>Join us for the 18+ sketchbook swap!<br>Bring your own sketchbook, drawing materials, and a reference of your character(s) and get ready to draw with other furry folks!	2026-01-17 02:30:00	2026-01-17 04:15:00	11	3	f	\N
60	2025-12-19 16:13:55	2025-12-24 22:00:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	60	Bleat Meet!	Calling all deer kind! Come join us for a meet and greet with members of the herd. Bring your best bleat and make some new hooved friends!	2026-01-16 22:00:00	2026-01-16 22:45:00	2	8	f	\N
94	2025-12-19 16:19:08	2025-12-24 22:03:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	94	Pokémon Card Trading Panel	Looking to snag that brand new card you've been chasing after?! Well this is the panel for you! Come join us in a fun, friendly, and fair environment full of Pokémon connoisseur's just like you!	2026-01-17 02:30:00	2026-01-17 03:45:00	9	8	f	\N
111	2025-12-19 16:22:14	2025-12-24 22:03:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	111	The Artist's Train: Speed Networking	Number one, you share your art. Number two, they share their art. Number three, swap and repeat until the timer is up! Welcome to “The Artist’s Train”, a wacky way to meet and learn about your fellow artists in a “speed networking” format.	2026-01-17 13:00:00	2026-01-17 13:45:00	13	3	f	\N
105	2025-12-19 16:20:33	2025-12-24 22:47:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	105	Let's $ew! a Stallion (part) (18+)	$15 per studentMEDIUM skill level.18+ ONLY!You ASKED for this. Just for the record, this is NOT my fault. This mid-level sewing class is WHAT THE FURRIES ASKED FOR. So we're doing it. We're going to sew a stallion. Part of a stallion. The level of detail on your finished.. stallion.. depends entirely on how FOCUSED you can be!	2026-01-17 04:00:00	2026-01-17 05:45:00	3	3	f	\N
188	2025-12-19 16:33:49	2025-12-24 22:42:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	188	Moc Shop: Build Your Own Bionicle Moc!	 I will provide an abundance of Bionicle parts- you get to put them together into your own completely unique creation! (Or show off your own if you bring them)	2026-01-17 23:00:00	2026-01-18 00:15:00	11	3	f	\N
136	2025-12-19 16:25:15	2025-12-24 22:05:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	136	Content Creator Meetup	Do you consider yourself a content creator? Whether it be Youtube, Twitch, Tiktok, or just a content creator in general? Well come and join your fellow furs who also work in the same field! Big or small, everyone is welcome!	2026-01-17 17:00:00	2026-01-17 17:45:00	7	8	f	\N
49	2025-12-19 16:12:38	2025-12-24 21:59:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	49	Ham Furs: Animals on Amateur Radio	Are you radio-curious, maybe you've bought one of those $30 software-defined radios and have been doing some listening in? Are you a seasoned ham, keying up on 7,200 kHz and telling all who will listen about your upcoming medical appointments? Either way, this panel should have something for you. Come learn and discuss new ways to test the limits of electronics, the atmosphere, and your landlord/HOA/significant other's patience. This panel will start with a brief presentation on the basics of amateur radio and then transition to a meetup. 73 de KC1WYV	2026-01-16 21:00:00	2026-01-16 21:45:00	2	2	f	\N
134	2025-12-19 16:25:02	2025-12-24 22:15:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	134	Let's $ew! Cheez Crackers	$10.00 per student.All skill levels welcomeAges 13+, or bring a parent or guardian, please.Who's hungry for a mouthful of salty, cheddary goodness? Too bad! You can't eat THIS high-fiber, low-fat snack, but you CAN make your own! (and maybe a second if you're fast!)	2026-01-17 16:00:00	2026-01-17 17:45:00	9	3	f	\N
56	2025-12-19 16:13:26	2025-12-24 22:00:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	56	The Wild West of the Atom	From secret test sites to tomorrow’s reactors, the American West has always been the atomic frontier. This panel tracks how the region shaped nuclear weapons, fueled innovation with reactors like EBR-I and II, and quite literally supplied the fuel through uranium mining. We’ll also dive into how the West is once again leading the charge with next-gen reactors and SMR development. Saddle up—nuclear history started out here and it’s not done yet.	2026-01-16 21:30:00	2026-01-16 22:45:00	10	5	f	\N
55	2025-12-19 16:13:21	2025-12-24 21:59:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	55	Furry Jeopardy!	It's a furry-themed version of America's favorite TV quiz show of answers and questions. Cheer on your fellow furries and test your own knowledge as you play along! Contestants will be chosen from the audience with a qualifying exam. The winner will receive an Amazon gift card and have their winning score converted into charity money.	2026-01-16 21:00:00	2026-01-16 22:30:00	12	7	f	\N
80	2025-12-19 16:17:26	2025-12-24 22:21:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	80	Animal Life Drawing Session & Educational Outreach	A unique opportunity for artists to conduct studies of living animal subjects! Potential subjects this year include a barred owl, great horned owl, red tail hawk, skunks, and a grey squirrel. The panel will be hosted by Maria (Wings of the Dawn), Alan (Wings Board Member), Ark the Fox (a wildlife management expert & Wings volunteer), and Robin ( or Deer Hudson Crafts - a Wings volunteer)! Please note there will be a $20 cover charge for entry. All entry fees will go directly to Wings to contribute to the care of rehab animals in need. Cash preferred, but card will also be accepted. This panel will also function as a Q&A while participants do their studies! Bring your own scribble supplies and drawing surfaces!	2026-01-17 00:00:00	2026-01-17 01:45:00	3	3	f	\N
62	2025-12-19 16:14:06	2025-12-24 22:00:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	62	Who Wants to be a Voice Actor?	From Bugs Bunny to Nick Wilde, anthropomorphic cartoon characters have been talking to us since we were kids. Have you ever wondered what goes into bringing those characters to life? Now you can participate! Join Timberpuppers for a fun dive into the voice acting profession. Whether you're a beginner, a hobbyist, or an aspiring professional, feel free to ask any questions relating to voiceover that you want an answer to! Then you can volunteer to perform dialogue yourself, where you'll receive direction and perform live for your fellow attendees! 	2026-01-16 22:00:00	2026-01-16 23:30:00	7	5	f	\N
71	2025-12-19 16:16:00	2025-12-24 22:01:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	71	Underground Communities: Asexuality, Furries, and the Internet	Incubating in the back corners of the internet, fighting for space and legitimacy, finding joy in community and visibility—in many ways, the history of the asexual community parallels the history of furry. From tiny underground publications and online message boards, through the struggle of being misunderstood and the relief of "I'm not the only one," the two communities share a common origin story. Join Lark from the Ace Archive project to learn about this overlooked chapter of queer history and what it can teach us about building authentic community.	2026-01-16 23:00:00	2026-01-17 00:15:00	6	2	f	\N
84	2025-12-19 16:18:00	2025-12-24 22:02:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	84	Fursuit Making 101	Making a fursuit is hard, and where do you start? Lets talk about the basics of fursuit making and things to consider such as materials, tools, skills, and time management. After the main presentation stick around to talk with other makers to share advice and get ideas.	2026-01-17 01:00:00	2026-01-17 02:15:00	7	3	f	\N
75	2025-12-19 16:16:54	2025-12-24 22:01:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	75	Boy Band Furs Meet Up	We’ve all been there! Listening to their music, picking our favorite guys, hanging posters on our walls, collecting their merchandise, going to concerts whenever possible, you name it! Whether you’re a fan of the classics such as New Kids on the Block, Backstreet Boys, and NSYNC, or you like groups of the last decade such as Big Time Rush and One Direction, and even the KPOP idols of today like BTS and Seventeen, you’re sure to enjoy this panel! Feel free to wear your favorite boy band shirts or bring some memorabilia to show off! 	2026-01-17 00:00:00	2026-01-17 00:45:00	9	8	f	\N
81	2025-12-19 16:17:40	2025-12-24 22:13:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	81	Welcome 2 Heck	Sinners of all species are invited to this revamped review of the underworld as depicted in anthro media, from vintage toons and furry comics to Hazbin Hotel season 2! This panel uses funny cartoon animals as a way to analyze the intersection of furry, queerness and theology, in an unholy fusion of a TED Talk and group therapy session. Meet fellow imps and discover what it means when anthros end up in The Bad Place.	2026-01-17 00:30:00	2026-01-17 01:15:00	10	10	f	\N
73	2025-12-19 16:16:26	2025-12-22 14:00:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	73	Dragon's Dojo	Everyone loves a good dance battle, Anthro New England’s floor wars style event is Dragon’s Dojo! Get ready to watch some of the best dancers in the fandom go head to head in a one on one dance battle, right here in Boston! We’re proud to work with our friends over at MAJIC Records to put on this wonderful combination of creativity, culture and music! <br>Every year we choose a theme that all dancers are expected to incorporate into their segments – we aim to emphasize not just the technicality of dance but also the connection to the music!	2026-01-16 23:00:00	2026-01-17 01:30:00	15	9	f	\N
112	2025-12-19 16:22:21	2025-12-24 22:03:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	112	Equine Meetup - All Things Horsey	This is a meetup up for the fellow equines, from mules to unicorns, all horsies and friends are welcome! Join us for some coloring and horsie talk with fellow equine friends!	2026-01-17 13:00:00	2026-01-17 13:45:00	11	8	f	\N
198	2025-12-19 16:35:08	2025-12-24 22:43:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	198	Queer Ghost Hunting 101	Is there an inherent connection between The Queer and The Strange? Do we love ghosts, cryptids and magic because it exists on the margins like we do, or is there something deeper to it? Join award-winning paranormal investigator Dash Kwiatkowski (Liminaltv.com) to learn some fun introductory paranormal investigation techniques and hear about the intrinsic connection between queer culture and the unknown, before closing out the workshop by working together to conduct a paranormal investigation of the space!	2026-01-18 00:00:00	2026-01-18 01:15:00	6	2	f	\N
121	2025-12-19 16:23:29	2025-12-24 22:04:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	121	Furry Western Cartoons	Continuing the tradition from the late Anthrocoon, this panel discusses and views clips from Western-related animated films, TV shows and commercials starring anthropomorphic characters mostly from the 1940s onward. They're partly what helped us get into furry.	2026-01-17 14:30:00	2026-01-17 15:15:00	6	10	f	\N
182	2025-12-19 16:32:59	2025-12-24 22:42:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	182	Bunny (And Bunny-Adjacent) Meetup	A panel for any bunnies or similar critters to meet up and hang out! Featuring rabbit trivia and a group photo.	2026-01-17 22:00:00	2026-01-17 22:45:00	11	8	f	\N
185	2025-12-19 16:33:18	2025-12-24 22:42:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	185	Armor Academy: Scale Armor Gauntlet	 Have you ever wanted to make your own scale mail? During this workshop, we will teach you the basics of making scale armor, and leave you with a scale gauntlet made by your own hands. There will be a 25 dollar materials fee to participate, but any are welcome to watch.	2026-01-17 22:30:00	2026-01-17 23:45:00	9	3	f	\N
199	2025-12-19 18:31:28	2025-12-24 22:43:31	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	199	New England Nostalgia Trip: Commercials!	Time to make the donuts? Boston's favorite pizza? IS Watercountry a very cool spot? Come on by to see a showcase of vintage New England TV commercials!	2026-01-18 00:30:00	2026-01-18 01:15:00	7	10	f	\N
109	2025-12-19 16:21:58	2025-12-24 22:14:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	109	Furry Im-purr-ov!	Come on down and participate in this whirlwind of improv comedy with your friendly feline host Heathclaw. All levels of experience are encouraged- a quick rundown of improv basics is all you'll need to start performing in a wide variety of improv games, including a few fursuit friendly fan favorites! Come prepared to make stuff up, act silly, laugh, and most importantly, have fun!While this panel aims to keep content at a PG-13 rating, due to the unpredictable nature of live improv, viewer discretion is advised.	2026-01-17 05:30:00	2026-01-17 07:15:00	2	10	f	\N
186	2025-12-19 16:33:28	2025-12-24 22:17:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	186	Data Structures and Algorithms	Embrace the wild west const and explore the new dominance frontier! Let's take a travelling salesperson tour of computer science puns!	2026-01-17 22:30:00	2026-01-17 23:45:00	2	2	f	\N
175	2025-12-19 16:31:55	2025-12-24 22:41:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	175	Blackjack and Hoofers: Hoofer Bleat and Greet	This is a meetup for all hoofers.If you got horns or hooves, come on by!	2026-01-17 21:30:00	2026-01-17 22:15:00	2	8	f	\N
177	2025-12-19 16:32:07	2025-12-24 22:41:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	177	Furry Logic's Escape Room Time Slot 10	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 21:30:00	2026-01-17 23:00:00	16	7	f	\N
191	2025-12-19 16:34:17	2025-12-24 22:42:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	191	Furry Logic's Escape Room Time Slot 11	Furry Logic is proud to bring back Cyberpunk Escape, previously showcased at conventions in 2020-2021! Players work together to untangle the corporate conspiracies and expose Megacorp for its wrongdoings from the inside - but act swiftly since the jamming drones can only protect you for so long before you are exposed to be traced and erased! Games are 60 minutes long, and spots will be limited! Tickets are $30 per person, with a portion of all tickets supporting charity! Want to just play with family or friends? Book a private room by paying for a minimum of 5 tickets in an empty slot. Please check out our online booking website https://www.bookeo.com/furrylogicllc to get your tickets early, or, check out our sales table on site for more info! If you have any questions, reach out to us at furrylogicllc@gmail.com	2026-01-17 23:00:00	2026-01-18 00:30:00	16	7	f	\N
108	2025-12-19 16:21:36	2025-12-24 22:47:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	108	Photo Room: Rubber Animals (18+)	Join us for a special after-hours opening of the Photo Room designed for extremely premium rubber photos! We'll have experts available to help shine your rubber gear and make you brighter than the sun. 18+ only. ID is required. (Please visit the ANE website for more details.)	2026-01-17 05:00:00	2026-01-17 06:00:00	1	1	f	\N
133	2025-12-19 16:24:54	2025-12-24 22:23:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	133	Build-an-Ear Workshop	Come join us to make and decorate your very own poseable fluffy ears! This is a beginner-friendly workshop with no sewing involved!	2026-01-17 16:00:00	2026-01-17 17:45:00	3	3	f	\N
114	2025-12-19 16:22:36	2025-12-24 22:03:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	114	Fashion Fursuiters Catwalk	Are you a fursuiter and fashionista? Does your fursuit have the type of fit that deserves the red carpet treatment? Well you're in luck! Here's your opportunity to walk the runway and get some extra photos/video of your outfit to match! This event is open to both Fashionable Fursuiters who want to walk the "Cat" Walk and spectators who want to see them show off! Join us on Telegram to workshop outfits and get hype for the show: https://t.me/+fwqIms_3t0Q2M2Ux 	2026-01-17 13:00:00	2026-01-17 14:15:00	10	8	f	\N
146	2025-12-19 16:27:27	2025-12-24 22:38:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	146	Wizard101 Furs Meet Up!	For those who have a love for Wizard101, or are potentially curious about the game! Come meet other fans of the game and find your next reason to clear a slot in your friend's list! We plan to have different activities set for everyone to engage in :)	2026-01-17 18:00:00	2026-01-17 18:45:00	9	8	f	\N
116	2025-12-19 16:22:49	2025-12-24 22:04:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	116	Wildfire Yoga - Spine and Shoulders	Join me, Awe Tiger, for a 3 part journey into yoga, meditation, and functional movement theory.  Each day will have a different focus - Day 1: Hip and Knee Health.  Day 2: Shoulders, Spine and Breath.  Day 3: Paws and Purrception - Quadrobics Conditioning.  Come explore these traditional health and spirituality based eastern practices meant to free one to the possibility of living in alignment with nature and unleashing the power of their animal body.  All classes will offer options for those with limited mobility who wish to practice from a chair. 	2026-01-17 13:00:00	2026-01-17 14:45:00	8	2	f	\N
129	2025-12-19 16:24:25	2025-12-24 22:04:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	129	The History of EDM and Rave Culture (+ Music Workshop!)	Headbangers, unite! Do you want to learn about all the little events that led to the best weekend of your life, partying hard underground with bass in your ears? My name is Mykra, and I plan to deliver a comprehensive history of the origins of EDM. We'll go back to the 90s all the way up to present day, discussing how raggae artists, synthesizer engineers, and a few really creative homebodies laid the foundation for the greatest music of our time. And after that, I'll deliver a few secret techniques on how I make my own EDM, and how you can get started!	2026-01-17 15:30:00	2026-01-17 16:45:00	7	11	f	\N
276	2025-12-19 18:44:21	2025-12-24 22:35:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	276	Furries at my University?!?!?!?	Looking for furries at your university, especially in America's college town, this is the meet for you! We will have a quick 10 minute presentation on how to start a group for those interested, and the rest of the time will be dedicated to a quick QnA followed by a meet and greet for the rest of the panel duration. The invitation to meet your follow furs also goes out to any propspective, current, and alumni for boston schools and any others!	2026-01-18 19:00:00	2026-01-18 20:15:00	11	8	f	\N
143	2025-12-19 16:27:13	2025-12-24 22:06:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	143	Winter Snowsports Panel	Come meet with other furs who are experienced or even just interested in snowsports!  Doesn't matter if you're a beginner or an expert, local or just visiting, we'll try to have something for everyone!	2026-01-17 17:30:00	2026-01-17 18:15:00	8	8	f	\N
144	2025-12-19 16:27:19	2025-12-24 22:06:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	144	Furs of Color!	(Previously called "POC furs meet!) This is a space for BIPOC (Black, Asian, Hispanic, Indigenous, and people of color ) to meet one another and share their fandom experiences, both good and bad. This will be a safe space for panelists to discuss our  experiences, literature, music, culture, food and so much more! Attendees will be free to participate and share their art & do some of the “make a badge” activities that will be provided by the heads of the panel :)! REMINDER! You don't need to be a Person of Color to join! Just be mindful of the conversations happening in this space! :)	2026-01-17 17:30:00	2026-01-17 18:45:00	11	8	f	\N
164	2025-12-19 16:30:26	2025-12-24 22:15:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	164	Make a Lil Guy: Character Design Workshop	How do people make such distinct, creative fursonas? Join this workshop discussing character design principles for furries, exploring everything from rough concept to final design, and all the fun steps in between! Designed for folks of all skill levels, and will also discuss tools that can help non-drawing types. (Note-taking/sketching tools are not necessary, but highly recommended!)	2026-01-17 20:00:00	2026-01-17 21:15:00	6	3	f	\N
152	2025-12-19 16:28:15	2025-12-24 22:39:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	152	Making Space with Makerspaces	Do you want to create your own merch or grow your artistic skills? Are you craving a creative third space or a rally point for your community? What you need is a makerspace!Learn how to tap into the Maker Mindset and take advantage of tools, resources, and classes available at makerspaces across the land.Led by Fabville’s project manager Christopher Ryerson with speakers from Artisans Asylum and  Local Furry makers! 	2026-01-17 18:30:00	2026-01-17 19:45:00	6	2	f	\N
125	2025-12-19 16:23:58	2025-12-24 22:15:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	125	Boston Gaymer Info	Boston Gaymers is a local non-profit group focused around building safe community spaces around gaming and and other nerdy activities. Join us for a get to knowus/Q&A session if you are interested in our meetups throughout the Boston area.	2026-01-17 15:00:00	2026-01-17 15:45:00	8	8	f	\N
157	2025-12-19 16:29:35	2025-12-24 22:39:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	157	Science Furs Meeting	A meet up for furs studying, employed, or generally interested in the sciences, mathematics, or engineering.  Come meet critters you’ll know you have at least two things in common with!  A great chance to share interests that make you wag, plus an opportunity to network with others in your field or get advice if you’re looking for it. 	2026-01-17 19:00:00	2026-01-17 20:15:00	10	8	f	\N
160	2025-12-19 16:29:59	2025-12-24 22:39:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	160	Taiko Drum Tournament	What is Taiko? It's a very fun Japanese style Drum rhythm game!  Arcade size drums, Suiter friendly Game play and all ages are welcome! (Under 16 will need adult accompaniment) Both Beginner and Veteran brackets available.  Sign up in the Video game room.	2026-01-17 19:00:00	2026-01-17 21:00:00	19	7	f	\N
215	2025-12-19 18:34:21	2025-12-24 22:19:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	215	Pawstar Playhouse	Pawstar Playhouse invites you to a lively gameshow where your favorite furries face off in hilarious and exciting challenges. From charades and furry trivia to creative drawing contests, every round promises laughs and surprises. Cheer on the contestants as they compete for fun prizes, and don’t miss your chance to win as part of the audience. Join the fun and experience a playful celebration of furry fandom like never before! 	2026-01-18 03:00:00	2026-01-18 04:15:00	13	10	f	\N
251	2025-12-19 18:41:16	2025-12-24 22:20:08	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	251	Where the River Meets the Sea - Mythology of the Mermaid	This panel is a survey of mythology surrounding "mermaids" and mermaid-like creatures. Half human, half fish, these beings exist between worlds and play a variety of roles in the stories in which they appear. Where and when did the concept of a "mermaid" arise, and how has it changed over the centuries? Join us as we discuss the tropes and how mermaids (and their close cousins) have appeared in literature, pop culture, and folklore.	2026-01-18 16:00:00	2026-01-18 05:15:00	13	3	f	\N
288	2025-12-19 18:45:38	2025-12-24 22:34:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	288	Kaiju Clash	When one furry kaiju meets another, naturally they will do battle and destroy the city around them. Oh, and you get to be the audience!  Explanation for staff: I'm building a small city out of foam and cardboard and I'm going to make fursuiters go WWE on it and smash it to pieces, repeatedly. 	2026-01-18 21:00:00	2026-01-18 22:45:00	10	10	f	\N
262	2025-12-19 18:42:46	2025-12-24 23:02:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	262	Fandom and Fursuiting	Do you like fursuiting? How about cosplay? The two hobbies aren't as different as you may think at first glance. Join Blush and Jedi as they talk about what happens when fursuiting and cosplay overlap, and why people enjoy crossing their favorite characters with their original characters. Whether you like to dress up as a critter from Disney, or you want your fursona to show off your love of a fandom, this is the panel for you.	2026-01-18 17:30:00	2026-01-18 18:45:00	10	2	f	\N
242	2025-12-19 18:39:53	2025-12-24 23:05:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	242	Costume Design for Performance	Excited about building or comissioning a costume but aren't sure where to start? What can we learn from the horror stories of costumes gone awry? Come learn the answers from Raeburn's many years of performance experience. 	2026-01-18 14:30:00	2026-01-18 15:45:00	2	5	f	\N
\.


--
-- Data for Name: links; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.links (id, created_at, updated_at, created_by, updated_by, nc_order, name, url) FROM stdin;
1	2025-12-19 09:16:03	\N	us1wm95uilu3p7mh	\N	1	Policies	https://www.anthronewengland.com/policies/
2	2025-12-19 09:16:24	2025-12-19 09:16:25	us1wm95uilu3p7mh	us1wm95uilu3p7mh	2	Accessibility Information	https://www.anthronewengland.com/venue/accessibility-information/
3	2025-12-19 09:16:46	\N	us1wm95uilu3p7mh	\N	3	Getting Here	https://www.anthronewengland.com/venue/getting-here/
4	2025-12-19 09:17:30	2025-12-27 03:38:17	us1wm95uilu3p7mh	us1wm95uilu3p7mh	4	Telegram	https://t.me/Anthro_NE
\.


--
-- Data for Name: locations; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.locations (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
7	2025-12-19 08:35:17	2025-12-22 12:39:28	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	7	Alcott Panelroom
11	2025-12-19 08:41:33	2025-12-22 12:39:33	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	11	Douglass Panelroom
9	2025-12-19 08:38:37	2025-12-22 12:39:45	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	9	Hancock Panelroom
2	2025-12-19 01:32:20	2025-12-22 12:39:51	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	2	Stone Panelroom
5	2025-12-19 01:42:33	2025-12-22 12:40:02	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	5	Harbor Ballroom
10	2025-12-19 08:40:04	2025-12-22 12:40:06	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	10	Commonwealth Ballroom
13	2025-12-19 08:47:19	2025-12-22 12:40:33	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	13	Paine Panelroom
8	2025-12-19 08:37:00	2025-12-22 12:41:15	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	8	Revere Panelroom
19	2025-12-22 12:21:44	2025-12-22 12:21:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	18	Burroughs Room
6	2025-12-19 01:44:16	2025-12-24 23:00:33	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	6	Webster Panelroom
1	2025-12-19 01:29:01	2025-12-22 12:35:27	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	1	Faneuil Room
12	2025-12-19 08:44:26	2025-12-22 12:36:03	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	12	Carlton Panelroom
16	2025-12-19 09:02:44	2025-12-22 12:37:19	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	16	Adams Room
4	2025-12-19 01:36:07	2025-12-22 12:33:30	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	4	Marina Ballroom
15	2025-12-19 09:01:21	2025-12-22 12:38:11	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	15	Grand Ballroom
3	2025-12-19 01:34:29	2025-12-22 12:38:50	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	3	Otis Panelroom
17	2025-12-19 09:07:36	2025-12-22 12:14:34	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	17	Grand Ballroom Prefunction
\.


--
-- Data for Name: pages; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.pages (id, created_at, updated_at, created_by, updated_by, nc_order, title, body, files) FROM stdin;
1	2025-12-19 09:19:17	2025-12-24 08:35:46	us1wm95uilu3p7mh	us1wm95uilu3p7mh	1	About This App	**This is not the official ANE schedule!**\n\nThis app is a beta of [FanJam](https://fanjam.live), a free event planning app for conventions. This schedule may not be up to date and may contain errors. You can find the official ANE 2026 schedule [here](https://ane2026.sched.com).\n\nThank you for being a beta tester! If you have any feedback, I would love to hear it. You can message me here:\n\n- **Email:** lark@fanjam.live\n- **Telegram:** @justlark	\N
\.


--
-- Data for Name: people; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.people (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
36	2025-12-22 12:43:06	2025-12-22 12:43:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	36	Ryuno
37	2025-12-22 12:43:25	2025-12-22 12:43:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	37	EnderFloofs
5	2025-12-19 01:38:55	2025-12-19 01:38:56	us1wm95uilu3p7mh	us1wm95uilu3p7mh	5	Tall Dog
38	2025-12-22 12:43:37	2025-12-22 12:43:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	38	Lirie the Great Dane
2	2025-12-19 01:32:30	2025-12-19 01:44:22	us1wm95uilu3p7mh	us1wm95uilu3p7mh	2	ANE Panels Leads
6	2025-12-19 08:33:57	2025-12-19 08:33:58	us1wm95uilu3p7mh	us1wm95uilu3p7mh	6	formless_chaos
7	2025-12-19 08:35:31	2025-12-19 08:35:31	us1wm95uilu3p7mh	us1wm95uilu3p7mh	7	Tianlong
65	2025-12-22 12:50:46	2025-12-22 13:11:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	65	Lark
9	2025-12-19 08:38:50	2025-12-19 08:38:50	us1wm95uilu3p7mh	us1wm95uilu3p7mh	9	FangsAkaCharlie
10	2025-12-19 08:40:15	2025-12-19 08:40:15	us1wm95uilu3p7mh	us1wm95uilu3p7mh	10	Red
11	2025-12-19 08:40:24	2025-12-19 08:40:24	us1wm95uilu3p7mh	us1wm95uilu3p7mh	11	Kariyoke
4	2025-12-19 01:36:16	2025-12-22 13:48:30	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	4	ANE Staff
52	2025-12-22 12:47:55	2025-12-22 12:47:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	52	Protogenesis
51	2025-12-22 12:47:39	2025-12-22 13:44:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	51	Seppi Schneider
17	2025-12-19 08:47:31	2025-12-19 08:47:32	us1wm95uilu3p7mh	us1wm95uilu3p7mh	17	Speedy
18	2025-12-19 08:48:30	2025-12-19 08:48:30	us1wm95uilu3p7mh	us1wm95uilu3p7mh	18	Train Nerd Zen
39	2025-12-22 12:43:52	2025-12-22 12:43:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	39	Hether Leijon
19	2025-12-19 08:51:42	2025-12-19 08:51:42	us1wm95uilu3p7mh	us1wm95uilu3p7mh	19	Equus Silvermane
20	2025-12-19 08:53:00	2025-12-19 08:53:01	us1wm95uilu3p7mh	us1wm95uilu3p7mh	20	Khalifa
21	2025-12-19 08:54:06	2025-12-19 08:54:06	us1wm95uilu3p7mh	us1wm95uilu3p7mh	21	Cat
22	2025-12-19 08:54:13	2025-12-19 08:54:13	us1wm95uilu3p7mh	us1wm95uilu3p7mh	22	Loaf
23	2025-12-19 08:55:32	2025-12-19 08:55:32	us1wm95uilu3p7mh	us1wm95uilu3p7mh	23	Marcoangelo
72	2025-12-22 12:52:59	2025-12-22 13:42:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	72	Wings Wildlife Rehabilitation
24	2025-12-19 08:58:59	2025-12-19 08:58:59	us1wm95uilu3p7mh	us1wm95uilu3p7mh	24	Winter Warburton
25	2025-12-19 09:00:09	2025-12-19 09:00:09	us1wm95uilu3p7mh	us1wm95uilu3p7mh	25	Denali
27	2025-12-19 09:04:05	2025-12-19 09:04:05	us1wm95uilu3p7mh	us1wm95uilu3p7mh	27	Nachie
61	2025-12-22 12:49:59	2025-12-22 12:50:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	61	TigerFennec
53	2025-12-22 12:48:05	2025-12-22 12:48:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	53	Blizzard
13	2025-12-19 08:43:04	2025-12-22 13:48:22	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	13	Atlas / Polaris
30	2025-12-19 16:08:32	2025-12-19 16:08:32	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	30	BearicTheCleric
31	2025-12-22 12:40:51	2025-12-22 12:40:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	31	Lucid
32	2025-12-22 12:41:27	2025-12-22 12:41:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	32	Blue
33	2025-12-22 12:42:28	2025-12-22 12:42:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	33	Kaage Ryoko
68	2025-12-22 12:51:52	2025-12-22 13:16:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	68	Giza White Mage
35	2025-12-22 12:42:53	2025-12-22 12:42:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	35	Nisha
54	2025-12-22 12:48:16	2025-12-22 12:48:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	54	Artie
41	2025-12-22 12:44:35	2025-12-22 12:44:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	41	Hopeful Monster
3	2025-12-19 01:34:41	2025-12-22 13:48:54	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	3	Alyxe Khei
71	2025-12-22 12:52:45	2025-12-22 13:48:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	71	ANE Charity Department
42	2025-12-22 12:45:16	2025-12-22 12:45:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	42	Adelair
43	2025-12-22 12:45:34	2025-12-22 12:45:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	43	RottenSkye
44	2025-12-22 12:45:46	2025-12-22 12:45:46	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	44	Iris N.
34	2025-12-22 12:42:41	2025-12-22 12:58:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	34	Mother Inferior
46	2025-12-22 12:46:10	2025-12-22 12:46:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	46	Scoot
47	2025-12-22 12:46:25	2025-12-22 12:46:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	47	Lyle the Otter
48	2025-12-22 12:46:35	2025-12-22 12:46:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	48	Goldie
40	2025-12-22 12:44:09	2025-12-22 12:46:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	40	Cirrus
49	2025-12-22 12:47:02	2025-12-22 12:47:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	49	U-235 Oxide
67	2025-12-22 12:51:35	2025-12-22 12:51:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	67	Taylor The Wolf
50	2025-12-22 12:47:25	2025-12-22 12:47:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	50	Wulfums
1	2025-12-19 01:29:12	2025-12-22 13:39:01	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	1	ANE Photo Team
55	2025-12-22 12:48:27	2025-12-22 12:48:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	55	Triton
56	2025-12-22 12:48:46	2025-12-22 12:48:46	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	56	TimberPuppers
84	2025-12-22 12:56:14	2025-12-22 13:29:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	84	Space Mouse Buzz
57	2025-12-22 12:49:05	2025-12-22 12:49:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	57	Doktor
58	2025-12-22 12:49:20	2025-12-22 12:49:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	58	RenRaccoon
60	2025-12-22 12:49:45	2025-12-22 12:49:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	60	Fig!
62	2025-12-22 12:50:12	2025-12-22 12:50:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	62	RaspberryLucario
63	2025-12-22 12:50:22	2025-12-22 12:50:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	63	Talstorm
64	2025-12-22 12:50:33	2025-12-22 12:50:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	64	sci
12	2025-12-19 08:41:49	2025-12-24 23:00:49	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	12	Kamen The Lycanroc!
45	2025-12-22 12:45:58	2025-12-22 12:58:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	45	Sophie Raccoon
8	2025-12-19 08:37:10	2025-12-22 13:33:43	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	8	Awe Tiger
66	2025-12-22 12:51:19	2025-12-22 12:51:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	66	Ferdinand Ferret
69	2025-12-22 12:52:10	2025-12-22 12:52:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	69	Marco Dvorzchak
70	2025-12-22 12:52:21	2025-12-22 12:52:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	70	Daniel 'Kast
59	2025-12-22 12:49:35	2025-12-22 12:52:28	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	59	Junir the Lion
29	2025-12-19 09:09:57	2025-12-22 13:23:02	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	29	Rick Fox
15	2025-12-19 08:44:35	2025-12-22 13:43:56	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	15	Toast
73	2025-12-22 12:53:19	2025-12-22 12:53:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	73	Vaz
28	2025-12-19 09:06:21	2025-12-22 13:06:34	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	28	Fletcher
14	2025-12-19 08:43:12	2025-12-22 13:48:34	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	14	Flint
95	2025-12-22 12:59:42	2025-12-22 12:59:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	95	cowdog
75	2025-12-22 12:53:48	2025-12-22 12:53:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	75	darlingpup
76	2025-12-22 12:54:10	2025-12-22 12:54:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	76	Nitral
77	2025-12-22 12:54:47	2025-12-22 12:54:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	77	Pibby
78	2025-12-22 12:54:58	2025-12-22 12:54:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	78	LilacPupp
79	2025-12-22 12:55:11	2025-12-22 12:55:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	79	Sturmovik
81	2025-12-22 12:55:31	2025-12-22 12:55:32	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	81	Varga
82	2025-12-22 12:55:44	2025-12-22 12:55:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	82	Lucifurr
121	2025-12-22 13:06:05	2025-12-22 13:40:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	121	Cosmo / The Stupendium
85	2025-12-22 12:56:33	2025-12-22 12:56:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	85	Burst
86	2025-12-22 12:56:53	2025-12-22 12:56:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	86	Fruitkitty
87	2025-12-22 12:57:38	2025-12-22 12:57:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	87	Chaz Hoss
88	2025-12-22 12:57:48	2025-12-22 12:57:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	88	Zera
129	2025-12-22 13:08:38	2025-12-22 13:08:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	129	Ori the Lynx
90	2025-12-22 12:58:24	2025-12-22 12:58:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	90	Harbee
91	2025-12-22 12:58:38	2025-12-22 12:58:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	91	Banjo
83	2025-12-22 12:55:56	2025-12-22 13:33:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	83	Lush BNB
93	2025-12-22 12:59:15	2025-12-22 12:59:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	93	Felix
16	2025-12-19 08:45:52	2025-12-22 13:34:54	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	16	Raeburn Rabbit
96	2025-12-22 12:59:51	2025-12-22 12:59:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	96	Art
97	2025-12-22 13:00:08	2025-12-22 13:00:08	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	97	Equine Meetup
98	2025-12-22 13:00:20	2025-12-22 13:00:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	98	Icky
99	2025-12-22 13:00:33	2025-12-22 13:00:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	99	Carbonitty
100	2025-12-22 13:01:07	2025-12-22 13:01:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	100	Rowan K9-Dragon
101	2025-12-22 13:01:18	2025-12-22 13:01:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	101	Kino
102	2025-12-22 13:01:35	2025-12-22 13:01:35	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	102	Cloverkit
103	2025-12-22 13:01:50	2025-12-22 13:01:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	103	Badgerbuck
105	2025-12-22 13:02:14	2025-12-22 13:02:14	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	105	Just Right Pup
107	2025-12-22 13:02:46	2025-12-22 13:02:46	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	107	Animation
108	2025-12-22 13:02:53	2025-12-22 13:02:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	108	VFX
109	2025-12-22 13:03:06	2025-12-22 13:03:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	109	Cyan Glaciertooth
131	2025-12-22 13:09:00	2025-12-22 13:44:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	131	Axigenix
124	2025-12-22 13:07:11	2025-12-22 13:25:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	124	CondensedNuts
112	2025-12-22 13:03:43	2025-12-22 13:03:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	112	Ganymede
113	2025-12-22 13:03:57	2025-12-22 13:03:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	113	Juni
114	2025-12-22 13:04:10	2025-12-22 13:04:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	114	Yatchi
115	2025-12-22 13:04:24	2025-12-22 13:04:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	115	Pozzum
116	2025-12-22 13:04:32	2025-12-22 13:04:32	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	116	Abby Tiger
117	2025-12-22 13:04:43	2025-12-22 13:04:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	117	Mykra
118	2025-12-22 13:04:57	2025-12-22 13:04:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	118	Bug
119	2025-12-22 13:05:07	2025-12-22 13:05:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	119	Jayay
120	2025-12-22 13:05:40	2025-12-22 13:05:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	120	Stag
111	2025-12-22 13:03:31	2025-12-22 13:39:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	111	MasterofPlush
94	2025-12-22 12:59:32	2025-12-22 13:20:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	94	Lilith
106	2025-12-22 13:02:37	2025-12-22 13:06:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	106	3D Modeling
122	2025-12-22 13:06:26	2025-12-22 13:06:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	122	Corvin
74	2025-12-22 12:53:36	2025-12-22 13:30:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	74	Crackerjack!
125	2025-12-22 13:07:23	2025-12-22 13:07:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	125	Talcott
126	2025-12-22 13:07:41	2025-12-22 13:07:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	126	JAYSTOR
127	2025-12-22 13:07:51	2025-12-22 13:07:51	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	127	hyenabbq
128	2025-12-22 13:08:13	2025-12-22 13:08:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	128	Ryoko Amesapphi
132	2025-12-22 13:09:06	2025-12-22 13:44:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	132	FJ
146	2025-12-22 13:12:34	2025-12-22 13:46:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	146	Cliff
133	2025-12-22 13:09:21	2025-12-22 13:09:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	133	Euphoria
134	2025-12-22 13:09:31	2025-12-22 13:09:31	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	134	duncekat
135	2025-12-22 13:09:43	2025-12-22 13:09:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	135	PYRAKUNEM
136	2025-12-22 13:10:02	2025-12-22 13:10:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	136	Ripley
137	2025-12-22 13:10:17	2025-12-22 13:10:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	137	Slewfoot the Saber
138	2025-12-22 13:10:35	2025-12-22 13:10:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	138	Daedalus
139	2025-12-22 13:10:47	2025-12-22 13:10:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	139	Denali/Eclipse
140	2025-12-22 13:10:58	2025-12-22 13:10:58	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	140	Straggler
141	2025-12-22 13:11:11	2025-12-22 13:11:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	141	Fabville
80	2025-12-22 12:55:22	2025-12-22 13:21:47	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	80	Majik
142	2025-12-22 13:11:34	2025-12-22 13:11:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	142	Huntertanuki
143	2025-12-22 13:11:55	2025-12-22 13:11:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	143	DjMuffinTops
144	2025-12-22 13:12:04	2025-12-22 13:12:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	144	Energyscarf
145	2025-12-22 13:12:24	2025-12-22 13:12:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	145	Maul
104	2025-12-22 13:02:02	2025-12-22 13:47:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	104	Zak Wolf
123	2025-12-22 13:06:54	2025-12-22 13:12:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	123	erasableData
147	2025-12-22 13:12:52	2025-12-22 13:12:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	147	Coda Mazur
226	2025-12-22 13:42:23	2025-12-22 13:42:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	226	Liam
89	2025-12-22 12:58:00	2025-12-22 13:23:08	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	89	Rito Bandito
92	2025-12-22 12:59:04	2025-12-22 13:32:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	92	Heath
110	2025-12-22 13:03:17	2025-12-22 13:43:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	110	Cenzozo
130	2025-12-22 13:08:46	2025-12-22 13:44:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	130	Voltage, the Lynx
149	2025-12-22 13:13:19	2025-12-22 13:13:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	149	Vought
148	2025-12-22 13:13:03	2025-12-22 13:13:42	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	148	Dusk Blackheart
151	2025-12-22 13:13:53	2025-12-22 13:13:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	151	DapperCap
152	2025-12-22 13:14:05	2025-12-22 13:14:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	152	BunnyBytezz
150	2025-12-22 13:13:36	2025-12-22 13:13:36	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	150	gamboiuwu
153	2025-12-22 13:14:37	2025-12-22 13:14:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	153	🐺 Feng
154	2025-12-22 13:14:55	2025-12-22 13:14:55	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	154	Maxy Wuff
190	2025-12-22 13:29:56	2025-12-22 13:29:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	190	Mr. Raccoon
156	2025-12-22 13:15:22	2025-12-22 13:15:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	156	Patches
157	2025-12-22 13:15:45	2025-12-22 13:15:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	157	Cryptid Codex
158	2025-12-22 13:15:59	2025-12-22 13:16:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	158	Siytron
159	2025-12-22 13:16:16	2025-12-22 13:16:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	159	TobiDeer
160	2025-12-22 13:16:26	2025-12-22 13:16:26	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	160	Nav
162	2025-12-22 13:17:18	2025-12-22 13:17:19	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	162	Ahanu The Hyena
163	2025-12-22 13:17:39	2025-12-22 13:17:39	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	163	SoftPauxs
165	2025-12-22 13:18:16	2025-12-22 13:40:09	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	165	Ivycomb
166	2025-12-22 13:18:25	2025-12-22 13:40:16	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	166	Shwabadi
167	2025-12-22 13:18:34	2025-12-22 13:40:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	167	CK9C
168	2025-12-22 13:18:44	2025-12-22 13:40:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	168	Silva Hound
217	2025-12-22 13:38:15	2025-12-22 13:46:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	217	gauthist
169	2025-12-22 13:19:18	2025-12-22 13:19:18	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	169	Murna
170	2025-12-22 13:19:29	2025-12-22 13:19:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	170	Rune
171	2025-12-22 13:19:40	2025-12-22 13:19:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	171	TrellianDuck
172	2025-12-22 13:20:02	2025-12-22 13:20:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	172	Bunsen Bitti
173	2025-12-22 13:20:15	2025-12-22 13:20:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	173	Unsigned Long
174	2025-12-22 13:20:33	2025-12-22 13:20:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	174	Windy Gote
175	2025-12-22 13:20:45	2025-12-22 13:20:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	175	Bionicle
155	2025-12-22 13:15:15	2025-12-22 13:20:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	155	Solvi
177	2025-12-22 13:22:10	2025-12-22 13:22:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	177	Befish
178	2025-12-22 13:22:32	2025-12-22 13:22:33	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	178	Queer Scouts, Inc.
179	2025-12-22 13:22:43	2025-12-22 13:22:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	179	Paranormal
180	2025-12-22 13:22:56	2025-12-22 13:22:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	180	Nightlinez
181	2025-12-22 13:23:30	2025-12-22 13:23:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	181	Astro
182	2025-12-22 13:23:44	2025-12-22 13:23:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	182	Remley
183	2025-12-22 13:24:02	2025-12-22 13:24:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	183	Mistress Nicole
184	2025-12-22 13:24:10	2025-12-22 13:24:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	184	Paris
191	2025-12-22 13:30:11	2025-12-22 13:30:11	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	191	Rye Pony
186	2025-12-22 13:24:38	2025-12-22 13:25:01	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	186	Blaz
187	2025-12-22 13:25:20	2025-12-22 13:25:20	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	187	Turbo
213	2025-12-22 13:37:29	2025-12-22 13:37:29	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	213	AztecTheSkullCat
189	2025-12-22 13:29:30	2025-12-22 13:29:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	189	TravisBeaw
192	2025-12-22 13:30:21	2025-12-22 13:30:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	192	Mztress Vega
193	2025-12-22 13:30:49	2025-12-22 13:30:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	193	Soggy
194	2025-12-22 13:31:07	2025-12-22 13:31:07	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	194	Marble Pupper
195	2025-12-22 13:31:22	2025-12-22 13:31:22	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	195	Dozer
196	2025-12-22 13:31:32	2025-12-22 13:31:32	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	196	Karnin
197	2025-12-22 13:31:40	2025-12-22 13:31:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	197	Daf
198	2025-12-22 13:31:53	2025-12-22 13:31:53	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	198	Hazard
199	2025-12-22 13:32:10	2025-12-22 13:32:10	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	199	Monster Momma
200	2025-12-22 13:32:21	2025-12-22 13:32:21	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	200	FenFen Fennec
201	2025-12-22 13:32:42	2025-12-22 13:49:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	201	ANE Events Team
202	2025-12-22 13:32:59	2025-12-22 13:32:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	202	Shiny Skunk
203	2025-12-22 13:33:08	2025-12-22 13:33:08	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	203	Patrick Totally
204	2025-12-22 13:33:37	2025-12-22 13:33:37	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	204	Lex
205	2025-12-22 13:33:56	2025-12-22 13:33:56	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	205	Mineral B
206	2025-12-22 13:34:06	2025-12-22 13:34:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	206	Roh!
207	2025-12-22 13:34:23	2025-12-22 13:34:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	207	Riiya
208	2025-12-22 13:34:48	2025-12-22 13:34:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	208	Dirk_has_rabies
188	2025-12-22 13:25:36	2025-12-22 13:35:03	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	188	Raz The Egret
209	2025-12-22 13:35:49	2025-12-22 13:35:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	209	Horizon
214	2025-12-22 13:37:40	2025-12-22 13:37:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	214	Guttah Pup
210	2025-12-22 13:35:59	2025-12-22 13:36:17	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	210	Leaf
211	2025-12-22 13:36:40	2025-12-22 13:36:41	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	211	Cartimum
212	2025-12-22 13:36:49	2025-12-22 13:36:50	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	212	Pen
215	2025-12-22 13:37:52	2025-12-22 13:37:52	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	215	Jedi
216	2025-12-22 13:38:02	2025-12-22 13:38:02	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	216	onyx's asmr
218	2025-12-22 13:38:30	2025-12-22 13:38:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	218	Sym Stenops
219	2025-12-22 13:38:43	2025-12-22 13:38:43	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	219	magicalArmageddon
26	2025-12-19 09:02:52	2025-12-22 13:44:36	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	26	Furry Logic
220	2025-12-22 13:39:11	2025-12-22 13:39:12	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	220	Juneberry
221	2025-12-22 13:39:25	2025-12-22 13:39:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	221	Kamen The Lycanroc!
222	2025-12-22 13:39:49	2025-12-22 13:39:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	222	BlueJediForce
164	2025-12-22 13:18:07	2025-12-22 13:40:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	164	Freeced
223	2025-12-22 13:40:59	2025-12-22 13:40:59	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	223	Rick
224	2025-12-22 13:41:45	2025-12-22 13:41:45	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	224	Randy Ringtail
225	2025-12-22 13:42:13	2025-12-22 13:42:13	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	225	cowsounds
185	2025-12-22 13:24:26	2025-12-22 13:47:05	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	185	Yve
161	2025-12-22 13:16:56	2025-12-22 13:45:30	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	161	Nostson
227	2025-12-22 13:42:49	2025-12-22 13:42:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	227	Capriccio Meinl
228	2025-12-22 13:42:59	2025-12-22 13:43:00	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	228	Thatfurrygamerfox
229	2025-12-22 13:43:24	2025-12-22 13:43:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	229	Ringgar
230	2025-12-22 13:43:40	2025-12-22 13:43:40	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	230	Yuril
231	2025-12-22 13:43:48	2025-12-22 13:43:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	231	Wotokay
176	2025-12-22 13:21:57	2025-12-22 13:44:25	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	176	Fedora Fennec
232	2025-12-22 13:45:15	2025-12-22 13:45:15	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	232	Maple Bartzie
233	2025-12-22 13:45:24	2025-12-22 13:45:24	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	233	Frost Nicoletti
234	2025-12-22 13:45:48	2025-12-22 13:45:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	234	Fiyerose
235	2025-12-22 13:45:57	2025-12-22 13:45:57	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	235	Maggie
236	2025-12-22 13:46:26	2025-12-22 13:46:27	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	236	Retro Destructo
237	2025-12-22 13:46:38	2025-12-22 13:46:38	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	237	Curio
238	2025-12-22 13:46:48	2025-12-22 13:46:48	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	238	Tommy Bean
239	2025-12-22 13:47:23	2025-12-22 13:47:23	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	239	BetaEtaDelota
240	2025-12-22 13:47:34	2025-12-22 13:47:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	240	Hope Collie
241	2025-12-22 13:47:44	2025-12-22 13:47:44	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	241	Monoid
242	2025-12-22 13:47:54	2025-12-22 13:47:54	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	242	Fast-Fox
243	2025-12-22 13:48:03	2025-12-22 13:48:04	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	243	KatFox
244	2025-12-24 22:27:06	2025-12-24 22:27:06	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	244	Ari
\.


--
-- Data for Name: tags; Type: TABLE DATA; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

COPY pjb8x7vqtomqrms.tags (id, created_at, updated_at, created_by, updated_by, nc_order, name) FROM stdin;
2	2025-12-19 01:44:41	2025-12-24 22:53:38	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	2	Mature
3	2025-12-19 08:57:25	2025-12-24 22:24:24	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	3	All Ages ($ Pay)
4	2025-12-24 22:22:45	2025-12-24 22:24:34	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	4	Adult 18+ ($ pay)
1	2025-12-19 01:29:54	2025-12-24 23:06:24	us1wm95uilu3p7mh	uscvrzmw0wdonjf6	1	All Ages
5	2025-12-24 22:27:41	2025-12-24 22:49:49	uscvrzmw0wdonjf6	uscvrzmw0wdonjf6	5	Adult 18+
\.


--
-- Name: about_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.about_id_seq', 1, true);


--
-- Name: announcements_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.announcements_id_seq', 1, false);


--
-- Name: categories_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.categories_id_seq', 13, true);


--
-- Name: events_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.events_id_seq', 306, true);


--
-- Name: links_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.links_id_seq', 6, true);


--
-- Name: locations_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.locations_id_seq', 19, true);


--
-- Name: pages_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.pages_id_seq', 1, true);


--
-- Name: people_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.people_id_seq', 244, true);


--
-- Name: tags_id_seq; Type: SEQUENCE SET; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

SELECT pg_catalog.setval('pjb8x7vqtomqrms.tags_id_seq', 5, true);


--
-- Name: _nc_m2m_people_events _nc_m2m_people_events_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms._nc_m2m_people_events
    ADD CONSTRAINT _nc_m2m_people_events_pkey PRIMARY KEY (events_id, people_id);


--
-- Name: _nc_m2m_tags_events _nc_m2m_tags_events_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms._nc_m2m_tags_events
    ADD CONSTRAINT _nc_m2m_tags_events_pkey PRIMARY KEY (events_id, tags_id);


--
-- Name: about about_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.about
    ADD CONSTRAINT about_pkey PRIMARY KEY (id);


--
-- Name: announcements announcements_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.announcements
    ADD CONSTRAINT announcements_pkey PRIMARY KEY (id);


--
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (id);


--
-- Name: events events_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.events
    ADD CONSTRAINT events_pkey PRIMARY KEY (id);


--
-- Name: links links_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.links
    ADD CONSTRAINT links_pkey PRIMARY KEY (id);


--
-- Name: locations locations_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.locations
    ADD CONSTRAINT locations_pkey PRIMARY KEY (id);


--
-- Name: pages pages_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.pages
    ADD CONSTRAINT pages_pkey PRIMARY KEY (id);


--
-- Name: people people_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.people
    ADD CONSTRAINT people_pkey PRIMARY KEY (id);


--
-- Name: tags tags_pkey; Type: CONSTRAINT; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

ALTER TABLE ONLY pjb8x7vqtomqrms.tags
    ADD CONSTRAINT tags_pkey PRIMARY KEY (id);


--
-- Name: about_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX about_order_idx ON pjb8x7vqtomqrms.about USING btree (nc_order);


--
-- Name: announcements_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX announcements_order_idx ON pjb8x7vqtomqrms.announcements USING btree (nc_order);


--
-- Name: categories_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX categories_order_idx ON pjb8x7vqtomqrms.categories USING btree (nc_order);


--
-- Name: events_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX events_order_idx ON pjb8x7vqtomqrms.events USING btree (nc_order);


--
-- Name: fk_categories_events_akcebark89; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_categories_events_akcebark89 ON pjb8x7vqtomqrms.events USING btree (categories_id);


--
-- Name: fk_locations_events_0emb9do2t9; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_locations_events_0emb9do2t9 ON pjb8x7vqtomqrms.events USING btree (locations_id);


--
-- Name: fk_people_events_d7hu2z_hol; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_people_events_d7hu2z_hol ON pjb8x7vqtomqrms._nc_m2m_people_events USING btree (events_id);


--
-- Name: fk_people_events_l6tgypbmws; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_people_events_l6tgypbmws ON pjb8x7vqtomqrms._nc_m2m_people_events USING btree (people_id);


--
-- Name: fk_tags_events_8057u1cyum; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_tags_events_8057u1cyum ON pjb8x7vqtomqrms._nc_m2m_tags_events USING btree (tags_id);


--
-- Name: fk_tags_events_oyg0fs322t; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX fk_tags_events_oyg0fs322t ON pjb8x7vqtomqrms._nc_m2m_tags_events USING btree (events_id);


--
-- Name: links_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX links_order_idx ON pjb8x7vqtomqrms.links USING btree (nc_order);


--
-- Name: locations_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX locations_order_idx ON pjb8x7vqtomqrms.locations USING btree (nc_order);


--
-- Name: pages_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX pages_order_idx ON pjb8x7vqtomqrms.pages USING btree (nc_order);


--
-- Name: people_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX people_order_idx ON pjb8x7vqtomqrms.people USING btree (nc_order);


--
-- Name: tags_order_idx; Type: INDEX; Schema: pjb8x7vqtomqrms; Owner: sparklefish
--

CREATE INDEX tags_order_idx ON pjb8x7vqtomqrms.tags USING btree (nc_order);


--
-- PostgreSQL database dump complete
--

\unrestrict k5EzhTwczVc7QaTnpIO2jsruN1ZEdB4A5B1ZcrY9KZrGMkZMmNbrC9ci4iETTOg

