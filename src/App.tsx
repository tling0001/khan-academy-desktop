import {useEffect,useMemo,useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Home as HomeIcon,ClipboardList as ActivitiesTabIcon,Search as SearchIcon,Bookmark as BookmarkIcon} from "lucide-react";
import {UnitExperience} from "./UnitExperience";
import androidMenu from "./data/android-learn-menu.json";
import androidCourseIds from "./data/android-course-ids.json";
import {BookOpen,Compass,GraduationCap,ClipboardList,UserRound,Search,Bell,ChevronRight,ArrowUpRight,Menu,X,Sparkles,Clock3,Calculator,FlaskConical,BookText,Globe2,Palette,Code2,Heart,Settings,CircleHelp,LogIn,PanelLeftClose,PanelLeftOpen,Bookmark} from "lucide-react";
type Page="Home"|"Explore"|"Assignments"|"Bookmarks"|"Profile";
type Course={title:string;group:string;description?:string;href:string;courseId?:string;symbol?:string;theme?:string;units?:string};
type AndroidMenuGroup={group:string;courses:{title:string;href:string}[]};
type AndroidCourseIds=Record<string,string>;
const categoryTheme=(group:string)=>/Math/.test(group)?"lavender":/Science/.test(group)?"mint":/Computing/.test(group)?"blue":/Test prep/.test(group)?"gold":/English/.test(group)?"pink":/Social studies/.test(group)?"gold":/Economics/.test(group)?"green":/Life skills/.test(group)?"mint":"blue";
const categorySymbol=(group:string)=>/Math/.test(group)?"∑":/Science/.test(group)?"⚛":/Computing/.test(group)?"</>":/Test prep/.test(group)?"✎":/English/.test(group)?"Aa":/Social studies/.test(group)?"◷":/Economics/.test(group)?"$":"✦";
const courses:Course[]=(androidMenu as AndroidMenuGroup[]).flatMap(section=>section.courses.map(item=>({title:item.title,group:section.group,href:item.href,courseId:(androidCourseIds as AndroidCourseIds)[item.href],symbol:categorySymbol(section.group),theme:categoryTheme(section.group)})));

const nav:{label:Page;icon:any}[]=[{label:"Home",icon:BookOpen},{label:"Explore",icon:Compass},{label:"Assignments",icon:ClipboardList},{label:"Profile",icon:UserRound}];
const subjects=[{name:"Math",detail:"From early math to calculus.",icon:Calculator,theme:"lavender"},{name:"Science",detail:"Biology, chemistry, physics, and more.",icon:FlaskConical,theme:"mint"},{name:"Computing",detail:"Programming and computer science.",icon:Code2,theme:"blue"},{name:"Arts & humanities",detail:"History, grammar, and art history.",icon:Palette,theme:"gold"},{name:"Reading & language arts",detail:"Reading comprehension and language.",icon:BookText,theme:"pink"},{name:"Economics & finance",detail:"Markets, money, and the economy.",icon:Globe2,theme:"green"}];
const external=(path:string)=>window.open("https://www.khanacademy.org"+path,"_blank","noopener,noreferrer");
function App(){
const [page,setPage]=useState<Page>("Home"),[query,setQuery]=useState(""),[search,setSearch]=useState(false),[expanded,setExpanded]=useState(true),[course,setCourse]=useState<Course|null>(null),[saved,setSaved]=useState<string[]>([]);
const filtered=useMemo(()=>courses.filter(c=>(c.title+" "+c.group+" "+(c.description??"")).toLowerCase().includes(query.toLowerCase())),[query]);
const hideTopbar=page==="Explore"||Boolean(course);
const navigate=(p:Page)=>{setPage(p);setCourse(null);setQuery("");};
const openCourse=(c:Course)=>{if(!c.courseId){external(c.href);return;}setCourse(c);};
return <div className="shell"><aside className={"sidebar "+(expanded?"":"compact")}><button className="brand" onClick={()=>navigate("Home")}><img className="logo-image" src="/android-launcher.png" alt="" />{expanded&&<b>Khan Academy</b>}</button><div className="rule"/><nav>{nav.map(n=>{const I=n.icon;return <button key={n.label} className={"navitem "+(page===n.label?"active":"")} onClick={()=>navigate(n.label)} title={n.label}><I size={19}/>{expanded&&<span>{n.label}</span>}</button>})}</nav>{expanded&&<div className="sidebottom"><div className="rule"/><button className="navitem" onClick={()=>external("/about")}><CircleHelp size={18}/><span>About Khan Academy</span></button><div className="donate"><Heart size={19}/><b>For every student</b><p>Every child deserves the chance to learn, no matter what.</p><button onClick={()=>external("/donate")}>Support us <ArrowUpRight size={13}/></button></div><small>© 2026 Khan Academy</small></div>}<button className="collapse" onClick={()=>setExpanded(!expanded)}>{expanded?<PanelLeftClose size={17}/>:<PanelLeftOpen size={17}/>}</button></aside>
<main>{!hideTopbar&&<header className="topbar"><button className="mobilebrand" onClick={()=>navigate("Home")}><img className="logo-image" src="/android-launcher.png" alt="" />{page==="Home"?"Khan Academy":page==="Assignments"?"Activities":page}</button><div className="crumb">{course?<><span>Explore</span><ChevronRight size={14}/><b>{course.title}</b></>:page}</div><div className="actions">{page==="Home"?<button className="iconbtn home-settings" onClick={()=>external("/settings")} aria-label="Settings"><Settings size={21}/></button>:<button className="searchbtn" onClick={()=>setSearch(!search)}><Search size={17}/><span>Search for a course or topic</span><kbd>⌘ K</kbd></button>}<button className="iconbtn" onClick={()=>navigate("Assignments")} aria-label="Notifications"><Bell size={19}/></button><button className="signin" onClick={()=>external("/login")}>Sign in</button><button className="signup" onClick={()=>external("/signup")}>Sign up</button></div></header>}
{search&&<div className="searchpanel"><div className="searchfield"><Search size={18}/><input autoFocus placeholder="Search courses, skills, and topics" value={query} onChange={e=>{setQuery(e.target.value);setPage("Explore");setCourse(null)}}/><button onClick={()=>{setSearch(false);setQuery("")}}><X size={17}/></button></div><small>Search Khan Academy learning content</small>{query&&filtered.slice(0,7).map(c=><button className="result" key={c.title} onClick={()=>{openCourse(c);setSearch(false)}}><span className={"mini "+c.theme}>{c.symbol}</span><span><b>{c.title}</b><small>{c.group}</small></span><ChevronRight size={16}/></button>)}</div>}
<div className="scroll" key={course?.title||page}>{course?<CourseDetail course={course} saved={saved.includes(course.title)} toggleSave={()=>setSaved(s=>s.includes(course.title)?s.filter(x=>x!==course.title):[...s,course.title])} back={()=>setCourse(null)} backLabel={page==="Explore"?"Explore":page}/>:page==="Home"?<Home open={openCourse} goCourses={()=>navigate("Explore")} goExplore={()=>navigate("Explore")}/>:page==="Explore"?<Explore open={openCourse}/>:page==="Assignments"?<EmptyPage title="Activities" intro="Continue learning and see your activity on Khan Academy." icon={<ClipboardList size={34}/>} heading="Sign in to view your activities" text="Your learning activity and assigned work will appear here when you sign in." action="Sign in" onAction={()=>external("/login")}/>:page==="Bookmarks"?<Bookmarks saved={saved} open={openCourse}/>:<Profile onSignIn={()=>external("/login")}/>}</div><nav className="android-bottom-tabs" aria-label="Primary navigation">{[{label:"Home",icon:HomeIcon,page:"Home" as Page},{label:"Activities",icon:ActivitiesTabIcon,page:"Assignments" as Page},{label:"Explore",icon:SearchIcon,page:"Explore" as Page},{label:"Bookmarks",icon:BookmarkIcon,page:"Bookmarks" as Page}].map(tab=>{const I=tab.icon;const active=page===tab.page||(tab.page==="Explore"&&Boolean(course));return <button key={tab.label} className={active?"selected":""} aria-current={active?"page":undefined} onClick={()=>navigate(tab.page)}><span className="tab-icon"><I size={25} strokeWidth={active?2.3:1.8}/></span><span>{tab.label}</span></button>})}</nav></main></div>
}
function Heading({eyebrow,title,action,onAction}:{eyebrow?:string;title:string;action?:string;onAction?:()=>void}){return <div className="heading"><div>{eyebrow&&<small>{eyebrow}</small>}<h2>{title}</h2></div>{action&&<button onClick={onAction}>{action}<ChevronRight size={15}/></button>}</div>}
function Home({open,goCourses,goExplore}:{open:(c:Course)=>void;goCourses:()=>void;goExplore:()=>void}) {
  const [dismissed,setDismissed]=useState<string[]>([]);
  const dismiss=(key:string)=>setDismissed(items=>items.includes(key)?items:[...items,key]);
  return <div className="content android-home enter">
    <section className="android-upsell welcome-card">
      <div className="upsell-copy">
        <h2>Ready to start learning?</h2>
        <p>Sign in or sign up to save your progress as you work through Khan Academy courses.</p>
        <button className="primary" onClick={()=>external("/login")}>Sign in <ArrowUpRight size={15}/></button>
      </div>
      <div className="upsell-illustration welcome-illustration" aria-hidden="true">
        <div className="welcome-orbit"/><div className="welcome-book"><BookOpen size={72}/></div><span>✦</span><i>✳</i>
      </div>
    </section>

    <section className="android-home-card recent-lessons-card">
      <div className="android-card-header"><h2>Recent lessons</h2></div>
      <div className="android-empty-state"><span><Clock3 size={24}/></span><p>No recent lessons</p></div>
    </section>

    {!dismissed.includes("curriculum")&&<section className="android-upsell curriculum-card">
      <button className="dismiss-card" aria-label="Dismiss" onClick={()=>dismiss("curriculum")}><X size={16}/></button>
      <div className="upsell-copy">
        <h2>You can learn anything</h2>
        <p>Khan Academy is available in multiple languages and regions.</p>
        <button className="primary" onClick={()=>external("/settings/language")}>Select language <Globe2 size={15}/></button>
      </div>
      <div className="upsell-illustration curriculum-illustration" aria-hidden="true"><Globe2 size={76}/><span>文</span><i>Aa</i></div>
    </section>}

    {!dismissed.includes("kids")&&<section className="android-upsell kids-card">
      <button className="dismiss-card" aria-label="Dismiss" onClick={()=>dismiss("kids")}><X size={16}/></button>
      <div className="upsell-copy">
        <h2>Khan Academy Kids</h2>
        <p>Get our free, award-winning app for kids ages 2–8.</p>
        <button className="primary" onClick={()=>window.open("https://play.google.com/store/apps/details?id=org.khankids.android","_blank","noopener,noreferrer")}>Download now <ArrowUpRight size={15}/></button>
      </div>
      <div className="upsell-illustration kids-illustration" aria-hidden="true"><span>★</span><BookOpen size={65}/><i>♥</i></div>
    </section>}


  </div>;
}
function CourseCard({course,open}:{course:Course;open:()=>void}){return <button className="coursecard" onClick={open}><div className={"art "+course.theme}><span>{course.symbol||"✦"}</span><small>{course.group}</small></div><div className="cardbody"><small>{course.group}</small><b>{course.title}</b>{course.description&&<p>{course.description}</p>}<div>Open course<ChevronRight size={15}/></div></div></button>}
function Explore({open}:{open:(c:Course)=>void}){
 type MenuDomain={domainId:string;translatedTitle:string;icon?:string|null;children:{translatedTitle:string;href:string;courseId?:string}[]};
 const [term,setTerm]=useState(""),[recent,setRecent]=useState<string[]>([]),[expanded,setExpanded]=useState<string[]>([]);
 const [liveDomains,setLiveDomains]=useState<MenuDomain[]|null>(null),[menuError,setMenuError]=useState(""),[menuLoading,setMenuLoading]=useState(true);
 useEffect(()=>{let active=true;invoke<MenuDomain[]>("fetch_khan_explore_menu").then(domains=>{if(!active)return;if(domains.length)setLiveDomains(domains);else setMenuError("Khan Academy returned an empty curriculum menu.");setMenuLoading(false)}).catch(e=>{if(active){setMenuError(String(e));setMenuLoading(false)}});return()=>{active=false}},[]);
 const domains:MenuDomain[]=liveDomains??Array.from(new Set(courses.map(c=>c.group.split(":")[0].trim()))).map(name=>({domainId:name,translatedTitle:name,children:courses.filter(c=>c.group.split(":")[0].trim()===name).map(c=>({translatedTitle:c.title,href:c.href,courseId:c.courseId}))}));
 const items:Course[]=domains.flatMap(d=>d.children.map(c=>({title:c.translatedTitle,group:d.translatedTitle,href:c.href,courseId:c.courseId??(androidCourseIds as AndroidCourseIds)[c.href],symbol:categorySymbol(d.translatedTitle),theme:categoryTheme(d.translatedTitle)})));
 const filtered=items.filter(c=>(c.title+" "+c.group).toLowerCase().includes(term.toLowerCase()));
 const toggle=(id:string)=>setExpanded(v=>v.includes(id)?v.filter(x=>x!==id):[...v,id]);
 const choose=(c:Course)=>{setRecent(v=>[c.title,...v.filter(x=>x!==c.title)].slice(0,5));open(c)};
 const domainIcon=(icon?:string|null)=>icon?.startsWith("//")?"https:"+icon:icon??"";
 return <div className="content enter ka-explore">
  <div className="ka-explore-heading"><h1>Explore</h1></div>
  <label className="ka-explore-search"><Search size={19}/><input value={term} onChange={e=>setTerm(e.target.value)} placeholder="Search Khan Academy"/>{term&&<button onClick={()=>setTerm("")} aria-label="Clear search"><X size={16}/></button>}</label>
  {!term&&recent.length>0&&<section className="ka-recent-search"><b>Recent searches</b>{recent.map(x=><button key={x} onClick={()=>setTerm(x)}><Clock3 size={15}/>{x}</button>)}</section>}
  {!term&&<section className="ka-domains-card ka-android-card">
    <div className="ka-domains-header"><h2>Browse Khan Academy</h2><button onClick={()=>{const pick=items[Math.floor(Math.random()*items.length)];if(pick)choose(pick)}}>Surprise me</button></div>
    <div className="ka-domain-tiles">{domains.map(d=><button key={d.domainId} className="ka-domain-tile" onClick={()=>{const first=d.children[0];if(first)external(first.href)}}>{d.icon&&<img className="ka-domain-image" src={domainIcon(d.icon)} alt=""/>}<b>{d.translatedTitle}</b><ChevronRight size={17}/></button>)}</div>
  </section>}
  <section className="ka-domains-card ka-course-menu-card">
   <div className="ka-domains-header"><h2>Browse Khan Academy</h2></div>
   {term?<div className="ka-domain-courses">{filtered.map(c=><button key={c.href+c.title} onClick={()=>choose(c)}><span className={"ka-course-avatar "+c.theme}>{c.symbol}</span><span><b>{c.title}</b><small>{c.group}</small></span><ChevronRight size={18}/></button>)}</div>:
    <div className="ka-domain-list">{domains.map(d=>{const expandedDomain=expanded.includes(d.domainId);return <section key={d.domainId} className="ka-domain-section"><button className="ka-domain-row" onClick={()=>toggle(d.domainId)}>{d.icon?<img className="ka-domain-image" src={domainIcon(d.icon)} alt=""/>:<span className={"ka-course-avatar "+categoryTheme(d.translatedTitle)}>{categorySymbol(d.translatedTitle)}</span>}<b>{d.translatedTitle}</b><ChevronRight className={expandedDomain?"expanded":""} size={18}/></button>{expandedDomain&&<div className="ka-domain-submenu">{d.children.map(child=>{const c=items.find(item=>item.href===child.href&&item.title===child.translatedTitle);return <button key={child.href+child.translatedTitle} onClick={()=>c&&choose(c)}><span>{child.translatedTitle}</span><ChevronRight size={16}/></button>})}</div>}</section>})}</div>}
   {menuLoading&&<p className="ka-menu-status">Loading courses…</p>}{menuError&&<p className="ka-menu-status">Could not refresh the curriculum: {menuError}. Showing the Android 8.5.1 menu.</p>}
  </section>
  {term&&filtered.length===0&&<div className="empty large"><Search size={30}/><h2>No results</h2><p>Try another course or topic.</p></div>}
 </div>
}
function CourseDetail({course,saved,toggleSave,back,backLabel}:{course:Course;saved:boolean;toggleSave:()=>void;back:()=>void;backLabel:string}){return <UnitExperience course={course} saved={saved} toggleSave={toggleSave} back={back} backLabel={backLabel}/>;}
function EmptyPage({title,intro,icon,heading,text,action,onAction}:{title:string;intro:string;icon:any;heading:string;text:string;action:string;onAction:()=>void}){return <div className="content enter"><div className="intro"><small>YOUR LEARNING ACTIVITY</small><h1>{title}</h1><p>{intro}</p></div><div className="empty large">{icon}<h2>{heading}</h2><p>{text}</p><button className="primary" onClick={onAction}>{action} <ArrowUpRight size={15}/></button></div></div>}
function Bookmarks({saved,open}:{saved:string[];open:(c:Course)=>void}){const items=courses.filter(c=>saved.includes(c.title));return <div className="content enter"><div className="intro"><small>YOUR SAVED LEARNING</small><h1>Bookmarks</h1><p>Courses you’ve saved to come back to later.</p></div>{items.length?<div className="cards">{items.map(c=><CourseCard key={c.title} course={c} open={()=>open(c)}/>)}</div>:<div className="empty large"><Bookmark size={34}/><h2>No bookmarks yet</h2><p>Save a course from its course page and it will appear here.</p><button className="primary" onClick={()=>external("/bookmarks")}>Open Khan Academy bookmarks <ArrowUpRight size={15}/></button></div>}</div>}
function Profile({onSignIn}:{onSignIn:()=>void}){return <div className="content enter"><div className="intro"><small>YOUR KHAN ACADEMY</small><h1>Profile</h1><p>Manage your learning journey and account settings.</p></div><div className="profile"><UserRound size={32}/><div><b>Welcome to Khan Academy</b><p>Sign in or create an account to save your progress across devices.</p><button className="primary" onClick={onSignIn}><LogIn size={15}/> Sign in</button></div></div><div className="profilelinks"><div><Settings/><span><b>Settings</b><small>Manage your account preferences on Khan Academy.</small></span><ArrowUpRight/></div><div><Heart/><span><b>Our mission</b><small>A free, world-class education for anyone, anywhere.</small></span><ArrowUpRight/></div></div></div>}
export default App;