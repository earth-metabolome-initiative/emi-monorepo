//! Library providing a Rust Font-Awesome enumeration.
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
use strum::IntoEnumIterator;

#[derive(Debug, serde::Serialize)]
/// Struct representing a row in the icon CSV.
struct IconRow<'a> {
    /// Name of the icon.
    name: &'a str,
    /// Description of the icon.
    description: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, strum::EnumIter)]
/// Struct representing a Font Awesome icon.
pub enum Icon {
    /// A numeral 0, representing the number.
    Zero,
    /// A numeral 1, representing the number.
    One,
    /// A numeral 2, representing the number.
    Two,
    /// A numeral 3, representing the number.
    Three,
    /// A numeral 4, representing the number.
    Four,
    /// A numeral 5, representing the number.
    Five,
    /// A numeral 6, representing the number.
    Six,
    /// A numeral 7, representing the number.
    Seven,
    /// A numeral 8, representing the number.
    Eight,
    /// A numeral 9, representing the number.
    Nine,
    /// A lowercase letter 'a', representing the letter.
    A,
    /// An icon of a person in a wheelchair, indicating accessibility.
    AccessibleIcon,
    /// A book with a person's silhouette, representing a contact list.
    AddressBook,
    /// A card with a person's silhouette, indicating contact information.
    AddressCard,
    /// The logo of Algolia, representing the search engine.
    Algolia,
    /// Text aligned to the center, representing text formatting.
    AlignCenter,
    /// Text justified, representing text formatting.
    AlignJustify,
    /// Text aligned to the left, representing text formatting.
    AlignLeft,
    /// Text aligned to the right, representing text formatting.
    AlignRight,
    /// The logo of Alipay, representing the payment platform.
    Alipay,
    /// The logo of Amazon Pay, representing the payment service.
    AmazonPay,
    /// An anchor, representing stability or maritime themes.
    Anchor,
    /// An anchor with a circled check mark, representing stability.
    AnchorCircleCheck,
    /// An anchor with a circled exclamation mark, representing caution.
    AnchorCircleExclamation,
    /// An anchor with a circled `X`, representing instability.
    AnchorCircleXmark,
    /// An anchor with a lock, representing stability and security.
    AnchorLock,
    /// The logo of Android, an operating system for mobile devices.
    Android,
    /// A downward angle, representing direction.
    AngleDown,
    /// A left angle, representing direction.
    AngleLeft,
    /// A right angle, representing direction.
    AngleRight,
    /// An upward angle, representing direction.
    AngleUp,
    /// A downward angle, representing direction.
    AnglesDown,
    /// A left angle, representing direction.
    AnglesLeft,
    /// A right angle, representing direction.
    AnglesRight,
    /// An upward angle, representing direction.
    AnglesUp,
    /// The ankh symbol, representing life in ancient Egypt.
    Ankh,
    /// The logo of Apple, representing the tech company.
    Apple,
    /// The logo of Apple Pay, representing the payment service.
    ApplePay,
    /// A whole apple, representing the fruit.
    AppleWhole,
    /// An architectural arch, representing structure or gateways.
    Archway,
    /// An arrow pointing down, representing downward direction.
    ArrowDown,
    /// An arrow pointing down with numbers 1 to 9, representing sorting.
    ArrowDown19,
    /// An arrow pointing down with numbers 9 to 1, representing sorting in
    /// reverse.
    ArrowDown91,
    /// An arrow pointing down from A to Z, representing sorting.
    ArrowDownAZ,
    /// A long arrow pointing down, indicating downward direction.
    ArrowDownLong,
    /// A short wide arrow pointing down, indicating downward direction.
    ArrowDownShortWide,
    /// Arrows pointing down and up across a line, representing bidirectional
    /// movement.
    ArrowDownUpAcrossLine,
    /// An arrow pointing down and up with a lock, representing secure
    /// bidirectional movement.
    ArrowDownUpLock,
    /// A short wide arrow pointing down, indicating downward direction.
    ArrowDownWideShort,
    /// An arrow pointing down with letters Z to A, representing reverse
    /// alphabetical order.
    ArrowDownZA,
    /// An arrow pointing to the left, indicating direction or back.
    ArrowLeft,
    /// A long arrow pointing left, representing extended backward direction.
    ArrowLeftLong,
    /// An arrow pointer, representing a cursor or selection.
    ArrowPointer,
    /// A right arrow, representing forward direction.
    ArrowRight,
    /// Arrows pointing right and left, representing bidirectional movement.
    ArrowRightArrowLeft,
    /// A right arrow coming from a bracket, indicating exit.
    ArrowRightFromBracket,
    /// A long arrow pointing right, indicating extended forward direction.
    ArrowRightLong,
    /// A right arrow pointing to a bracket, indicating entry.
    ArrowRightToBracket,
    /// A right arrow pointing to a city, representing urban direction.
    ArrowRightToCity,
    /// An arrow rotating to the left, indicating undo or backward movement.
    ArrowRotateLeft,
    /// An arrow rotating to the right, indicating redo or forward movement.
    ArrowRotateRight,
    /// A downward trending arrow, representing decline.
    ArrowTrendDown,
    /// An upward trending arrow, representing growth or increase.
    ArrowTrendUp,
    /// An arrow turning down, representing downward movement.
    ArrowTurnDown,
    /// An arrow turning up, representing upward movement.
    ArrowTurnUp,
    /// An upward arrow, indicating upward direction or increase.
    ArrowUp,
    /// An arrow pointing up with numbers 1 to 9, representing sorting.
    ArrowUp19,
    /// An arrow pointing up with numbers 9 to 1, representing reverse sorting.
    ArrowUp91,
    /// An arrow pointing up with letters A to Z, representing sorting in
    /// alphabetical order.
    ArrowUpAZ,
    /// A bracket with an upward arrow, indicating upload or elevation.
    ArrowUpFromBracket,
    /// A ground water pump with an arrow pointing up, indicating water
    /// extraction.
    ArrowUpFromGroundWater,
    /// A water pump with an arrow pointing up, indicating water extraction.
    ArrowUpFromWaterPump,
    /// A long arrow pointing up, indicating upward direction.
    ArrowUpLong,
    /// An arrow pointing up and right with dots, representing movement or
    /// progression.
    ArrowUpRightDots,
    /// An arrow pointing up-right from a square, representing an external link.
    ArrowUpRightFromSquare,
    /// A short wide arrow pointing up, indicating upward direction.
    ArrowUpShortWide,
    /// A short wide arrow pointing up, indicating upward direction.
    ArrowUpWideShort,
    /// An arrow pointing up with letters Z to A, representing reverse
    /// alphabetical order.
    ArrowUpZA,
    /// Arrows pointing down to a line, representing downward movement.
    ArrowsDownToLine,
    /// Arrows pointing down to people, representing distribution or allocation.
    ArrowsDownToPeople,
    /// Arrows pointing left and right, representing bidirectional movement.
    ArrowsLeftRight,
    /// Arrows pointing left and right to a line, representing directional
    /// alignment.
    ArrowsLeftRightToLine,
    /// Rotating arrows, representing refresh or rotation.
    ArrowsRotate,
    /// Arrows in a spinning motion, representing rotation or refresh.
    ArrowsSpin,
    /// Arrows splitting up and left, representing divergence.
    ArrowsSplitUpAndLeft,
    /// Arrows pointing to a circle, representing centralization.
    ArrowsToCircle,
    /// Arrows pointing to a dot, representing convergence or focus.
    ArrowsToDot,
    /// Arrows pointing to an eye, representing focus or attention.
    ArrowsToEye,
    /// A set of arrows turning right, representing a directional change.
    ArrowsTurnRight,
    /// Arrows turning to dots, representing conversion or focus.
    ArrowsTurnToDots,
    /// Arrows pointing up and down, representing bidirectional movement.
    ArrowsUpDown,
    /// Arrows pointing in all directions, indicating movement or navigation.
    ArrowsUpDownLeftRight,
    /// Arrows pointing up to a line, indicating upward movement.
    ArrowsUpToLine,
    /// An asterisk, representing additional information or footnotes.
    Asterisk,
    /// The at symbol (@), representing email or social media.
    At,
    /// An atom, representing science or physics.
    Atom,
    /// A screen with sound waves, indicating audio description for the visually
    /// impaired.
    AudioDescription,
    /// The symbol for the Argentine austral, indicating currency.
    AustralSign,
    /// A medal, representing achievement or recognition.
    Award,
    /// The letter \"B\", representing the alphabet.
    B,
    /// A baby face, representing an infant.
    Baby,
    /// A baby carriage, representing childcare or infancy.
    BabyCarriage,
    /// An arrow pointing left, indicating backward or rewind.
    Backward,
    /// Fast backward arrows, representing rapid reverse.
    BackwardFast,
    /// A step backward symbol, representing reverse or undo.
    BackwardStep,
    /// A strip of bacon, representing food or breakfast.
    Bacon,
    /// Multiple bacteria, representing microbiology.
    Bacteria,
    /// A bacterium, representing microbiology.
    Bacterium,
    /// A shopping bag, representing commerce or shopping.
    BagShopping,
    /// The symbol for the Bahá'í Faith, representing the religion.
    Bahai,
    /// The symbol for the Thai baht, indicating currency.
    BahtSign,
    /// A circle with a slash, indicating prohibition.
    Ban,
    /// A cigarette with a ban symbol, representing no smoking.
    BanSmoking,
    /// A bandage, representing first aid or healing.
    Bandage,
    /// The symbol for the Bangladeshi taka, indicating currency.
    BangladeshiTakaSign,
    /// A barcode, representing scanning or product identification.
    Barcode,
    /// Three horizontal bars, indicating a menu or list.
    Bars,
    /// Bars showing progress, representing loading or progression.
    BarsProgress,
    /// Staggered bars, representing a progress indicator.
    BarsStaggered,
    /// A baseball, representing the sport.
    Baseball,
    /// A baseball bat and ball, representing the sport.
    BaseballBatBall,
    /// A shopping basket, representing retail or groceries.
    BasketShopping,
    /// A basketball, representing the sport.
    Basketball,
    /// A bathtub, representing bathing or bathrooms.
    Bath,
    /// An empty battery, representing no power.
    BatteryEmpty,
    /// A full battery, representing full charge or power.
    BatteryFull,
    /// A battery half full, representing moderate power.
    BatteryHalf,
    /// A battery one-quarter full, representing low power.
    BatteryQuarter,
    /// A battery three-quarters full, representing power.
    BatteryThreeQuarters,
    /// A bed, representing sleep or rest.
    Bed,
    /// A bed with a pulse line, representing healthcare or emergency.
    BedPulse,
    /// An empty beer mug, representing beverages.
    BeerMugEmpty,
    /// A ringing bell, indicating notifications or alerts.
    Bell,
    /// A concierge bell, representing service or assistance.
    BellConcierge,
    /// A bell with a slash, indicating no notifications.
    BellSlash,
    /// A Bézier curve, representing vector graphics or design.
    BezierCurve,
    /// An icon of a bicycle, representing cycling.
    Bicycle,
    /// A pair of binoculars, indicating search or exploration.
    Binoculars,
    /// A biohazard symbol, representing hazardous materials.
    Biohazard,
    /// The logo of Bitcoin, representing the cryptocurrency.
    Bitcoin,
    /// The symbol for Bitcoin, indicating cryptocurrency.
    BitcoinSign,
    /// A blender, representing kitchen appliances.
    Blender,
    /// A blender with a phone, representing multitasking or devices.
    BlenderPhone,
    /// A blog symbol, representing blogging or writing.
    Blog,
    /// The logo of Bluetooth, representing the wireless technology.
    Bluetooth,
    /// The logo of Bluetooth B, representing the wireless technology.
    BluetoothB,
    /// A bold 'B', representing bold text.
    Bold,
    /// A lightning bolt, representing speed or electricity.
    Bolt,
    /// A lightning bolt, representing electricity or energy.
    BoltLightning,
    /// An icon of a bomb, representing danger or explosive action.
    Bomb,
    /// A bone, representing the skeletal system or pet treats.
    Bone,
    /// A bong, representing smoking or cannabis use.
    Bong,
    /// An open book, representing reading or literature.
    Book,
    /// A book with maps, representing an atlas or geography.
    BookAtlas,
    /// A book representing the Bible, a holy book in Christianity.
    BookBible,
    /// A book with a bookmark, representing reading or saved pages.
    BookBookmark,
    /// A book representing the Journal of the Whills from Star Wars.
    BookJournalWhills,
    /// A medical book, representing healthcare knowledge.
    BookMedical,
    /// An open book, representing reading or literature.
    BookOpen,
    /// An open book with a user icon, representing reading or studying.
    BookOpenReader,
    /// A book representing the Quran, a holy book in Islam.
    BookQuran,
    /// A book with a skull, representing danger or mystery.
    BookSkull,
    /// A book representing the Tanakh, a canonical collection in Judaism.
    BookTanakh,
    /// A bookmark, indicating saved items or favorites.
    Bookmark,
    /// An icon representing all borders.
    BorderAll,
    /// A border with no lines, indicating no borders.
    BorderNone,
    /// An icon representing the top-left border.
    BorderTopLeft,
    /// A borehole, representing drilling or wells.
    BoreHole,
    /// A bottle with a droplet, representing liquid or moisture.
    BottleDroplet,
    /// A bottle of water, representing hydration.
    BottleWater,
    /// A bowl of food, representing dining.
    BowlFood,
    /// A bowl of rice, representing food.
    BowlRice,
    /// A bowling ball, representing the sport.
    BowlingBall,
    /// A simple box, representing a container.
    Box,
    /// A box with files, representing storage or archiving.
    BoxArchive,
    /// A box that is open, representing delivery or unboxing.
    BoxOpen,
    /// A box of tissues, representing healthcare or hygiene.
    BoxTissue,
    /// Packing boxes, representing moving or storage.
    BoxesPacking,
    /// Stacked boxes, representing storage or organization.
    BoxesStacked,
    /// Braille text, representing accessibility for the blind.
    Braille,
    /// A brain, representing intelligence or mental processes.
    Brain,
    /// The symbol for the Brazilian real, indicating currency.
    BrazilianRealSign,
    /// A slice of bread, representing food.
    BreadSlice,
    /// A simple bridge, representing infrastructure.
    Bridge,
    /// A bridge with a circled check mark, indicating an approved bridge.
    BridgeCircleCheck,
    /// A bridge with a circled exclamation mark, indicating a bridge with
    /// caution.
    BridgeCircleExclamation,
    /// A bridge with a circled `X`, indicating a closed bridge.
    BridgeCircleXmark,
    /// A bridge with a lock, representing security.
    BridgeLock,
    /// A bridge over water, representing infrastructure.
    BridgeWater,
    /// A briefcase, representing work or business.
    Briefcase,
    /// A briefcase with a medical cross, representing medical supplies.
    BriefcaseMedical,
    /// A broom, representing cleaning.
    Broom,
    /// A broom with a ball, representing cleaning or a sport.
    BroomBall,
    /// A brush, representing painting or art.
    Brush,
    /// The logo of Bitcoin, representing the cryptocurrency.
    Btc,
    /// A bucket, representing a container for liquids.
    Bucket,
    /// A bug, representing an insect or an error in software.
    Bug,
    /// A bug with a slash, indicating no bugs.
    BugSlash,
    /// Multiple bugs, representing software issues or pests.
    Bugs,
    /// A tall building, indicating construction or urban areas.
    Building,
    /// A building with a circled arrow pointing right, representing a building
    /// exit.
    BuildingCircleArrowRight,
    /// A building with a circled check mark, representing an approved building.
    BuildingCircleCheck,
    /// A building with a circled exclamation mark, representing a building with
    /// caution.
    BuildingCircleExclamation,
    /// A building with a circled `X`, representing a closed building.
    BuildingCircleXmark,
    /// A building with columns, representing classical architecture.
    BuildingColumns,
    /// A building with a flag, representing government or institution.
    BuildingFlag,
    /// A building with a lock, representing security.
    BuildingLock,
    /// A building with 'NGO', representing a non-governmental organization.
    BuildingNgo,
    /// A building with a shield, representing security.
    BuildingShield,
    /// A building with 'UN', representing the United Nations.
    BuildingUn,
    /// A building with a user icon, representing a workplace or office.
    BuildingUser,
    /// A building with wheat, representing agriculture or agribusiness.
    BuildingWheat,
    /// A bullhorn, representing announcements or public address.
    Bullhorn,
    /// A bullseye, representing a target or goal.
    Bullseye,
    /// A burger, representing food or fast food.
    Burger,
    /// An explosion or burst, representing impact or energy.
    Burst,
    /// A bus, representing public transportation.
    Bus,
    /// A simple bus, representing public transportation.
    BusSimple,
    /// A briefcase with a clock, representing business hours or time
    /// management.
    BusinessTime,
    /// A capital letter 'C', representing the letter.
    C,
    /// A cable car, representing a type of public transportation.
    CableCar,
    /// A cake with candles, representing celebration or birthday.
    CakeCandles,
    /// A calculator, representing mathematical calculations.
    Calculator,
    /// A simple calendar, representing scheduling.
    Calendar,
    /// A calendar with a check mark, representing a confirmed date.
    CalendarCheck,
    /// A calendar showing a day, representing scheduling.
    CalendarDay,
    /// A calendar with marked days, indicating a schedule or event.
    CalendarDays,
    /// A calendar with a minus sign, representing removing an event.
    CalendarMinus,
    /// A calendar with a plus sign, representing adding an event.
    CalendarPlus,
    /// A calendar with a week view, representing weekly schedule.
    CalendarWeek,
    /// A calendar with an `X`, representing a cancelled date.
    CalendarXmark,
    /// An icon of a camera, representing photography.
    Camera,
    /// An old-fashioned camera, indicating photography or photos.
    CameraRetro,
    /// A camera with a rotation arrow, representing photo orientation.
    CameraRotate,
    /// A campground symbol, representing camping or outdoor activities.
    Campground,
    /// A candy cane, representing Christmas or sweets.
    CandyCane,
    /// A cannabis leaf, representing the plant or its products.
    Cannabis,
    /// Two capsules, representing medication or supplements.
    Capsules,
    /// An icon of a car, indicating a vehicle or transportation.
    Car,
    /// A car battery, representing automotive power.
    CarBattery,
    /// A car with a burst, indicating accident or impact.
    CarBurst,
    /// A car with a key, indicating vehicle status.
    CarOn,
    /// The rear view of a car, representing transportation.
    CarRear,
    /// A side view of a car, indicating transportation.
    CarSide,
    /// A car in a tunnel, representing travel or transportation.
    CarTunnel,
    /// A caravan, representing travel or transportation.
    Caravan,
    /// A downward caret, representing dropdowns or more options.
    CaretDown,
    /// A caret pointing left, indicating backward direction.
    CaretLeft,
    /// A caret pointing right, indicating forward direction.
    CaretRight,
    /// An upward pointing caret, indicating expansion or scroll up.
    CaretUp,
    /// A carrot, representing the vegetable.
    Carrot,
    /// A shopping cart with a downward arrow, representing adding to cart.
    CartArrowDown,
    /// A flatbed cart, representing transportation or logistics.
    CartFlatbed,
    /// A flatbed cart with a suitcase, representing luggage transport.
    CartFlatbedSuitcase,
    /// A shopping cart with a plus sign, representing adding to cart.
    CartPlus,
    /// A shopping cart, representing commerce or shopping.
    CartShopping,
    /// A cash register, indicating point of sale or retail.
    CashRegister,
    /// A cat, representing the animal.
    Cat,
    /// The logo of CC Amazon Pay, representing the credit card payment service.
    CcAmazonPay,
    /// The logo of CC Amex, representing the credit card payment service.
    CcAmex,
    /// The logo of CC Apple Pay, representing the credit card payment service.
    CcApplePay,
    /// The logo of CC Diners Club, representing the credit card payment
    /// service.
    CcDinersClub,
    /// The logo of CC Discover, representing the credit card payment service.
    CcDiscover,
    /// The logo of CC JCB, representing the credit card payment service.
    CcJcb,
    /// The logo of `MasterCard`, indicating a credit card or payment.
    CcMastercard,
    /// The logo of CC `PayPal`, representing the credit card payment service.
    CcPaypal,
    /// The logo of CC Stripe, representing the credit card payment service.
    CcStripe,
    /// The logo of Visa credit card, indicating payment.
    CcVisa,
    /// The symbol for the Ghanaian cedi, indicating currency.
    CediSign,
    /// The symbol for cent, indicating currency.
    CentSign,
    /// A certificate, indicating achievement or certification.
    Certificate,
    /// A chair, representing seating or furniture.
    Chair,
    /// A chalkboard, representing teaching or education.
    Chalkboard,
    /// A chalkboard with a user icon, representing teaching or instruction.
    ChalkboardUser,
    /// Two champagne glasses clinking, representing celebration.
    ChampagneGlasses,
    /// A charging station, representing electric vehicle charging.
    ChargingStation,
    /// An area chart, representing data trends.
    ChartArea,
    /// A bar chart, representing data comparison.
    ChartBar,
    /// A column chart, representing data visualization.
    ChartColumn,
    /// A Gantt chart, representing project management.
    ChartGantt,
    /// A line chart, representing data trends.
    ChartLine,
    /// A pie chart, representing data visualization.
    ChartPie,
    /// A simple bar chart, representing data or statistics.
    ChartSimple,
    /// A check mark, symbolizing confirmation or success.
    Check,
    /// A double check mark, representing confirmation or approval.
    CheckDouble,
    /// A checkmark entering a slot, representing verification.
    CheckToSlot,
    /// A wedge of cheese, representing dairy or food.
    Cheese,
    /// A chess piece, representing the game of chess.
    Chess,
    /// A chess bishop, representing the game of chess.
    ChessBishop,
    /// A chess board, representing the game of chess.
    ChessBoard,
    /// A chess king, representing the game of chess.
    ChessKing,
    /// A chess knight, representing the game of chess.
    ChessKnight,
    /// A chess pawn, representing the game of chess.
    ChessPawn,
    /// A chess queen, representing the game of chess.
    ChessQueen,
    /// A chess rook, representing the game of chess.
    ChessRook,
    /// A downward chevron, representing a dropdown or more options.
    ChevronDown,
    /// A chevron pointing left, indicating backward direction.
    ChevronLeft,
    /// A chevron pointing right, indicating forward direction.
    ChevronRight,
    /// A chevron pointing upwards, indicating upward movement or navigation.
    ChevronUp,
    /// A child, indicating a young person.
    Child,
    /// A child holding a weapon, representing child soldiers.
    ChildCombatant,
    /// A child in a dress, representing a young girl.
    ChildDress,
    /// A child reaching out, representing assistance or curiosity.
    ChildReaching,
    /// Two children, representing youth or family.
    Children,
    /// A church building, representing a place of worship.
    Church,
    /// A simple circle, representing shape or completeness.
    Circle,
    /// A circle with an arrow pointing down, indicating downward movement.
    CircleArrowDown,
    /// A circle with an arrow pointing left, indicating backward movement.
    CircleArrowLeft,
    /// A circle with an arrow pointing right, indicating forward movement.
    CircleArrowRight,
    /// A circle with an arrow pointing up, indicating upward movement.
    CircleArrowUp,
    /// A check mark inside a circle, indicating confirmation.
    CircleCheck,
    /// A circle with a chevron pointing down, indicating downward direction.
    CircleChevronDown,
    /// A circle with a chevron pointing left, indicating backward direction.
    CircleChevronLeft,
    /// A circle with a chevron pointing right, indicating forward direction.
    CircleChevronRight,
    /// A circle with a chevron pointing up, indicating upward direction.
    CircleChevronUp,
    /// A circle with a dollar sign and slot, indicating payment.
    CircleDollarToSlot,
    /// A circle with a dot, indicating focus or selection.
    CircleDot,
    /// A downward arrow inside a circle, indicating scroll down or download.
    CircleDown,
    /// An exclamation mark inside a circle, indicating important information or
    /// alerts.
    CircleExclamation,
    /// A circle with an 'H', representing hospital.
    CircleH,
    /// A half-filled circle, representing partial loading or status.
    CircleHalfStroke,
    /// A circle with an 'i', representing information.
    CircleInfo,
    /// A circle with a left arrow, indicating backward direction.
    CircleLeft,
    /// A circle with a minus sign, indicating subtraction.
    CircleMinus,
    /// A circle with nodes, representing connections or network.
    CircleNodes,
    /// A circle with a notch, representing a loading or progress indicator.
    CircleNotch,
    /// A circle with a pause symbol, indicating media pause.
    CirclePause,
    /// A circle with a play symbol, indicating media playback.
    CirclePlay,
    /// A circle with a plus sign, indicating addition.
    CirclePlus,
    /// A circle with a question mark, indicating inquiry.
    CircleQuestion,
    /// A circle with a radiation symbol, indicating hazard.
    CircleRadiation,
    /// A circle with a right arrow, indicating forward direction.
    CircleRight,
    /// A circle with a stop symbol, indicating cessation.
    CircleStop,
    /// An upward arrow inside a circle, indicating scroll up or upload.
    CircleUp,
    /// A user icon inside a circle, indicating a user profile.
    CircleUser,
    /// A circled `X` mark, indicating closure or deletion.
    CircleXmark,
    /// A skyline of buildings, indicating an urban area or city.
    City,
    /// A clapperboard, representing filmmaking or production.
    Clapperboard,
    /// A clipboard, representing note-taking or data recording.
    Clipboard,
    /// A clipboard with a check mark, representing completed tasks.
    ClipboardCheck,
    /// A clipboard with a list, representing tasks or notes.
    ClipboardList,
    /// A clipboard with a question mark, representing inquiry or uncertainty.
    ClipboardQuestion,
    /// A clipboard with a user icon, representing user data or forms.
    ClipboardUser,
    /// A clock face, indicating time.
    Clock,
    /// A clock with an arrow rotating left, representing time reversal.
    ClockRotateLeft,
    /// Two overlapping squares, indicating duplication or cloning.
    Clone,
    /// A closed captioning symbol, representing subtitles or accessibility.
    ClosedCaptioning,
    /// A cloud, representing cloud storage or weather.
    Cloud,
    /// A cloud with a downward arrow, representing cloud download.
    CloudArrowDown,
    /// A cloud with an upward arrow, indicating upload to the cloud.
    CloudArrowUp,
    /// A cloud with a lightning bolt, representing a thunderstorm.
    CloudBolt,
    /// A cloud with meatballs, representing food or weather.
    CloudMeatball,
    /// A cloud with a moon, representing partly cloudy night.
    CloudMoon,
    /// A cloud with a moon and rain, representing nighttime rain.
    CloudMoonRain,
    /// A cloud with rain, representing weather or precipitation.
    CloudRain,
    /// A cloud with heavy rain, representing a downpour.
    CloudShowersHeavy,
    /// A cloud with water droplets, representing rain.
    CloudShowersWater,
    /// A cloud with a sun, representing partly cloudy weather.
    CloudSun,
    /// A cloud with sun and rain, representing mixed weather.
    CloudSunRain,
    /// The logo of Cloudflare, representing the web infrastructure company.
    Cloudflare,
    /// A clover, representing luck or St. Patrick's Day.
    Clover,
    /// An icon representing coding or programming.
    Code,
    /// A branch in code, indicating version control or branching.
    CodeBranch,
    /// A check mark, representing a code commit.
    CodeCommit,
    /// Two pieces of code being compared, indicating code review or comparison.
    CodeCompare,
    /// A forked path, representing branching in code.
    CodeFork,
    /// A symbol representing code merging.
    CodeMerge,
    /// A symbol representing a pull request in code versioning.
    CodePullRequest,
    /// The logo of `CodePen`, a social development environment for front-end
    /// designers and developers.
    Codepen,
    /// Coins, representing money or currency.
    Coins,
    /// A colon symbol, representing punctuation or separation.
    ColonSign,
    /// A speech bubble, indicating comments or communication.
    Comment,
    /// A speech bubble with a dollar sign, representing financial comments.
    CommentDollar,
    /// A speech bubble with dots, representing comments or conversation.
    CommentDots,
    /// A speech bubble with a medical cross, representing medical
    /// communication.
    CommentMedical,
    /// A speech bubble with a slash, indicating no comments.
    CommentSlash,
    /// A speech bubble with \"SMS\", representing text messaging.
    CommentSms,
    /// Multiple speech bubbles, indicating conversation or comments.
    Comments,
    /// A speech bubble with a dollar sign, representing financial discussions.
    CommentsDollar,
    /// A compact disc, representing media storage.
    CompactDisc,
    /// A compass, representing navigation or direction.
    Compass,
    /// A drafting compass, representing design or architecture.
    CompassDrafting,
    /// A compress icon, representing minimization.
    Compress,
    /// A desktop computer, representing computing or technology.
    Computer,
    /// A computer mouse, representing input device.
    ComputerMouse,
    /// A cookie, representing snacks or website tracking.
    Cookie,
    /// A bitten cookie, representing snacks or desserts.
    CookieBite,
    /// Two overlapping documents, indicating copying.
    Copy,
    /// A circled 'C', indicating copyright protection.
    Copyright,
    /// A couch, representing furniture or relaxation.
    Couch,
    /// A cow, representing the animal.
    Cow,
    /// A credit card, indicating payment or financial transactions.
    CreditCard,
    /// The logo of Critical Role, representing the web series.
    CriticalRole,
    /// An image crop icon, representing editing.
    Crop,
    /// A simple crop icon, representing image cropping.
    CropSimple,
    /// A cross, representing religion or medical aid.
    Cross,
    /// A crosshair, indicating targeting or precision.
    Crosshairs,
    /// A crow, representing the bird.
    Crow,
    /// A crown, representing royalty or achievement.
    Crown,
    /// A crutch, representing injury support.
    Crutch,
    /// The symbol for the Brazilian cruzeiro, indicating currency.
    CruzeiroSign,
    /// A 3D cube, representing geometry or structure.
    Cube,
    /// Multiple cubes, representing 3D objects.
    Cubes,
    /// Stacked cubes, representing building blocks.
    CubesStacked,
    /// A capital letter 'D', representing the letter.
    D,
    /// The logo of Dungeons & Dragons, representing the tabletop role-playing
    /// game.
    DAndD,
    /// The logo of D&D Beyond, representing the Dungeons & Dragons toolset.
    DAndDBeyond,
    /// A stack of disks, representing a database.
    Database,
    /// An arrow pointing left with a line, representing backspace.
    DeleteLeft,
    /// The logo of the Democratic Party, representing the political party.
    Democrat,
    /// A desktop computer, indicating computing or work.
    Desktop,
    /// The dharma wheel, representing Buddhism.
    Dharmachakra,
    /// A diagram showing the next step, representing progression.
    DiagramNext,
    /// A diagram showing predecessors, representing planning.
    DiagramPredecessor,
    /// A diagram showing a project, representing planning.
    DiagramProject,
    /// A diagram showing successors, representing progression.
    DiagramSuccessor,
    /// A diamond, representing luxury or value.
    Diamond,
    /// A diamond turned to the right, representing geometric shapes.
    DiamondTurnRight,
    /// A pair of dice, representing games or chance.
    Dice,
    /// A 20-sided die, representing tabletop gaming.
    DiceD20,
    /// A six-sided die, representing gaming or chance.
    DiceD6,
    /// Two dice showing five, representing chance or gaming.
    DiceFive,
    /// Two dice showing four, representing chance or gaming.
    DiceFour,
    /// Two dice showing one, representing chance or gaming.
    DiceOne,
    /// Two dice showing six, representing chance or gaming.
    DiceSix,
    /// Two dice showing three, representing chance or gaming.
    DiceThree,
    /// Two dice showing two, representing chance or gaming.
    DiceTwo,
    /// The logo of Discord, a chat and communication platform for gamers.
    Discord,
    /// A virus, representing illness.
    Disease,
    /// A computer display, representing screens or monitors.
    Display,
    /// A division sign, representing mathematical operations.
    Divide,
    /// A DNA strand, representing genetics.
    Dna,
    /// The logo of Docker, a platform for containerized applications.
    Docker,
    /// A dog, representing the animal.
    Dog,
    /// A dollar sign, indicating currency or money.
    DollarSign,
    /// A dolly, representing transport or moving.
    Dolly,
    /// The symbol for the Vietnamese dong, indicating currency.
    DongSign,
    /// A closed door, representing privacy or security.
    DoorClosed,
    /// An open door, indicating entry or exit.
    DoorOpen,
    /// A dove, representing peace.
    Dove,
    /// Arrows pointing down-left and up-right to a center, representing
    /// convergence.
    DownLeftAndUpRightToCenter,
    /// A long arrow pointing down, representing downward direction.
    DownLong,
    /// A downward arrow, typically used to indicate download actions.
    Download,
    /// A dragon, representing mythical creatures or fantasy.
    Dragon,
    /// A polygon, representing geometric shapes.
    DrawPolygon,
    /// The logo of Dribbble, a platform for showcasing design work.
    Dribbble,
    /// The logo of Dropbox, a cloud storage service.
    Dropbox,
    /// A droplet of water, representing liquid or fluidity.
    Droplet,
    /// A droplet with a slash, representing no water.
    DropletSlash,
    /// A drum, representing music.
    Drum,
    /// A steelpan drum, representing music.
    DrumSteelpan,
    /// A drumstick with a bite, representing food.
    DrumstickBite,
    /// A dumbbell, representing fitness or weightlifting.
    Dumbbell,
    /// A dumpster, representing waste disposal.
    Dumpster,
    /// A dumpster on fire, representing chaos or disaster.
    DumpsterFire,
    /// A dungeon, representing a prison or game environment.
    Dungeon,
    /// The letter \"E\", representing the alphabet.
    E,
    /// An ear with a slash, representing hearing impairment.
    EarDeaf,
    /// An ear with sound waves, representing listening.
    EarListen,
    /// A globe focusing on Africa, representing global presence.
    EarthAfrica,
    /// A globe focusing on the Americas, representing global reach.
    EarthAmericas,
    /// A globe focusing on Asia, representing global presence.
    EarthAsia,
    /// A globe focusing on Europe, representing global presence.
    EarthEurope,
    /// A globe focusing on Oceania, representing global presence.
    EarthOceania,
    /// An egg, representing food or Easter.
    Egg,
    /// An eject button, representing removal.
    Eject,
    /// An elevator, representing vertical transportation.
    Elevator,
    /// A horizontal ellipsis, representing more options.
    Ellipsis,
    /// A vertical ellipsis, representing more options.
    EllipsisVertical,
    /// A closed envelope, representing email or messages.
    Envelope,
    /// An envelope with a circled check, representing approved mail.
    EnvelopeCircleCheck,
    /// An open envelope, representing received message.
    EnvelopeOpen,
    /// An open envelope with text, representing received message.
    EnvelopeOpenText,
    /// Multiple envelopes, representing bulk mail.
    EnvelopesBulk,
    /// An equals sign, representing equality.
    Equals,
    /// An eraser, representing correction or deletion.
    Eraser,
    /// The logo of Ethereum, representing the cryptocurrency.
    Ethereum,
    /// An Ethernet port, representing network connectivity.
    Ethernet,
    /// The symbol for the euro, indicating currency.
    EuroSign,
    /// A large exclamation mark, indicating importance or alerts.
    Exclamation,
    /// An outward pointing arrows from a box, indicating expansion.
    Expand,
    /// An explosion, representing blast or impact.
    Explosion,
    /// An eye, indicating visibility or views.
    Eye,
    /// An eyedropper, representing precision or medical use.
    EyeDropper,
    /// An eye with low vision, representing visual impairment.
    EyeLowVision,
    /// An eye with a slash through it, indicating hidden or invisible content.
    EyeSlash,
    /// A capital letter 'F', representing the letter.
    F,
    /// An angry face, representing anger.
    FaceAngry,
    /// A dizzy face, representing confusion.
    FaceDizzy,
    /// A flushed face, representing embarrassment.
    FaceFlushed,
    /// A frowning face, representing sadness.
    FaceFrown,
    /// A frowning face with open mouth, representing sadness.
    FaceFrownOpen,
    /// A grimacing face, representing discomfort.
    FaceGrimace,
    /// A grinning face, representing happiness.
    FaceGrin,
    /// A grinning face with beams, representing joy.
    FaceGrinBeam,
    /// A grinning face with beam and sweat, representing relief.
    FaceGrinBeamSweat,
    /// A grinning face with hearts, representing love.
    FaceGrinHearts,
    /// A grinning face with squinted eyes, representing humor.
    FaceGrinSquint,
    /// A grinning face with squinting eyes and tears, representing laughter.
    FaceGrinSquintTears,
    /// A grinning face with stars, representing excitement.
    FaceGrinStars,
    /// A grinning face with tears, representing laughter.
    FaceGrinTears,
    /// A grinning face with tongue out, representing playfulness.
    FaceGrinTongue,
    /// A grinning face with tongue out and squinted eyes, representing
    /// silliness.
    FaceGrinTongueSquint,
    /// A grinning face with tongue out and wink, representing silliness.
    FaceGrinTongueWink,
    /// A wide grinning face, representing happiness.
    FaceGrinWide,
    /// A grinning face with a wink, representing playfulness.
    FaceGrinWink,
    /// A kissing face, representing affection.
    FaceKiss,
    /// A kissing face with beams, representing love.
    FaceKissBeam,
    /// A kissing face with a wink and heart, representing affection.
    FaceKissWinkHeart,
    /// A laughing face, representing humor.
    FaceLaugh,
    /// A laughing face with beams, representing joy.
    FaceLaughBeam,
    /// A laughing face with squinted eyes, representing humor.
    FaceLaughSquint,
    /// A laughing face with a wink, representing humor.
    FaceLaughWink,
    /// A meh face, representing indifference.
    FaceMeh,
    /// A blank face, representing indifference.
    FaceMehBlank,
    /// A face with rolling eyes, representing annoyance.
    FaceRollingEyes,
    /// A crying face, representing sadness or crying.
    FaceSadCry,
    /// A sad face with a tear, representing sadness or crying.
    FaceSadTear,
    /// A smiling face, indicating happiness or friendliness.
    FaceSmile,
    /// A smiling face with beams, representing joy.
    FaceSmileBeam,
    /// A smiling face with a wink, representing happiness or playfulness.
    FaceSmileWink,
    /// A surprised face, representing shock.
    FaceSurprise,
    /// A tired face, representing fatigue.
    FaceTired,
    /// The logo of Facebook, representing the social media platform.
    Facebook,
    /// A fan, representing cooling or ventilation.
    Fan,
    /// The logo of Fantasy Flight Games, representing the game publisher.
    FantasyFlightGames,
    /// A faucet, representing plumbing or water.
    Faucet,
    /// A faucet with a drip, representing water or plumbing.
    FaucetDrip,
    /// A fax machine, representing facsimile transmission.
    Fax,
    /// A feather, indicating lightness or writing.
    Feather,
    /// A pointed feather, representing writing or quill.
    FeatherPointed,
    /// A ferry boat, representing water transportation.
    Ferry,
    /// The logo of Figma, a design and prototyping tool.
    Figma,
    /// A simple document, indicating a file or document.
    File,
    /// A file with a downward arrow, representing file download.
    FileArrowDown,
    /// A file with an upward arrow, representing file upload.
    FileArrowUp,
    /// A file with an audio symbol, representing audio files.
    FileAudio,
    /// A file with a circled check, representing approved files.
    FileCircleCheck,
    /// A file with a circled exclamation mark, representing important files.
    FileCircleExclamation,
    /// A file with a circled minus, representing removed files.
    FileCircleMinus,
    /// A file with a circled plus, representing added files.
    FileCirclePlus,
    /// A file with a circled question mark, representing unknown files.
    FileCircleQuestion,
    /// A file with a circled `X`, representing deleted files.
    FileCircleXmark,
    /// A file with code, representing programming files.
    FileCode,
    /// A document with a signature line, indicating a contract or agreement.
    FileContract,
    /// A file with CSV text, representing CSV files.
    FileCsv,
    /// A document with the Excel logo, indicating a spreadsheet file.
    FileExcel,
    /// A document with an arrow pointing out, indicating file export.
    FileExport,
    /// A file with an image, representing image files.
    FileImage,
    /// A document with an arrow pointing in, indicating file import.
    FileImport,
    /// A document with an invoice, indicating billing or payments.
    FileInvoice,
    /// A file with a dollar sign, representing financial documents.
    FileInvoiceDollar,
    /// A file with lines, representing documents.
    FileLines,
    /// A file with a medical symbol, representing medical records.
    FileMedical,
    /// A file with a PDF symbol, representing a document.
    FilePdf,
    /// A file with a pen, representing editable documents.
    FilePen,
    /// A file with a `PowerPoint` symbol, representing presentations.
    FilePowerpoint,
    /// A file with a prescription symbol, representing medical records.
    FilePrescription,
    /// A file with a shield, representing secure documents.
    FileShield,
    /// A file with a signature, representing signed documents.
    FileSignature,
    /// A file with a video symbol, representing video files.
    FileVideo,
    /// A file with a waveform, representing audio files.
    FileWaveform,
    /// A file with a Word symbol, representing a document.
    FileWord,
    /// A file with a zipper, representing compressed files.
    FileZipper,
    /// A paint bucket pouring, representing filling or color.
    Fill,
    /// A paint bucket dripping, representing paint or color fill.
    FillDrip,
    /// A strip of film, representing movies or filming.
    Film,
    /// A funnel filter, representing filtration or sorting.
    Filter,
    /// A filter with a circled dollar sign, representing financial filtering.
    FilterCircleDollar,
    /// A filter with a circled X, representing filter removal.
    FilterCircleXmark,
    /// A fingerprint, representing identity or security.
    Fingerprint,
    /// A flame, representing fire or heat.
    Fire,
    /// A fire burner, representing heating or cooking.
    FireBurner,
    /// A fire extinguisher, representing safety equipment.
    FireExtinguisher,
    /// A curved flame, representing fire.
    FireFlameCurved,
    /// A simple flame, representing fire or heat.
    FireFlameSimple,
    /// A fish, representing the animal or aquatic life.
    Fish,
    /// A fish with fins, representing the animal or swimming.
    FishFins,
    /// A flag, indicating a nation or marking something important.
    Flag,
    /// A checkered flag, representing racing or completion.
    FlagCheckered,
    /// The flag of the USA, representing the United States of America.
    FlagUsa,
    /// A laboratory flask, representing science or experimentation.
    Flask,
    /// A flask and vial, representing science or experimentation.
    FlaskVial,
    /// A floppy disk, representing data storage.
    FloppyDisk,
    /// The symbol for the florin, indicating currency.
    FlorinSign,
    /// A folder, representing a collection of documents or files.
    Folder,
    /// A closed folder, representing file storage.
    FolderClosed,
    /// A folder with a minus sign, indicating removing files.
    FolderMinus,
    /// An open folder, indicating accessible files or documents.
    FolderOpen,
    /// A folder with a plus sign, indicating adding files.
    FolderPlus,
    /// A folder with a tree structure, representing organization.
    FolderTree,
    /// A capital letter 'A', representing typography or fonts.
    Font,
    /// The logo of Font Awesome, representing the icon set.
    FontAwesome,
    /// A football, representing the sport.
    Football,
    /// An arrow pointing right, indicating forward or next.
    Forward,
    /// Two arrows pointing forward, representing fast forward.
    ForwardFast,
    /// An arrow pointing forward with a vertical line, indicating step forward.
    ForwardStep,
    /// The symbol for the franc, indicating currency.
    FrancSign,
    /// A frog, representing the animal.
    Frog,
    /// A soccer ball, representing the sport of soccer.
    Futbol,
    /// A capital letter 'G', representing the letter.
    G,
    /// The logo of the Galactic Republic, representing the Star Wars faction.
    GalacticRepublic,
    /// The logo of the Galactic Senate, representing the Star Wars faction.
    GalacticSenate,
    /// A video game controller, representing gaming.
    Gamepad,
    /// A gas pump, representing fuel or energy.
    GasPump,
    /// A speedometer or gauge, representing measurement or speed.
    Gauge,
    /// A high gauge, representing high level or measurement.
    GaugeHigh,
    /// A simple gauge, representing measurement.
    GaugeSimple,
    /// A gauge with a high reading, representing high measurement.
    GaugeSimpleHigh,
    /// A gavel, representing law or auctions.
    Gavel,
    /// A gear, representing settings or machinery.
    Gear,
    /// Multiple gears, representing settings or machinery.
    Gears,
    /// A gemstone, representing jewelry or value.
    Gem,
    /// A genderless symbol, representing gender neutrality.
    Genderless,
    /// The logo of GG, representing good game.
    Gg,
    /// A circle with \"GG\", representing good game.
    GgCircle,
    /// A ghost, often used for spooky or playful themes.
    Ghost,
    /// A wrapped gift box, representing presents or surprises.
    Gift,
    /// Two wrapped gifts, representing presents or surprises.
    Gifts,
    /// The logo of GitHub, representing the code hosting platform.
    Github,
    /// A glass of water, representing hydration.
    GlassWater,
    /// A glass of water with a droplet, representing hydration.
    GlassWaterDroplet,
    /// A pair of glasses, representing vision or eyewear.
    Glasses,
    /// A globe, representing the world or global reach.
    Globe,
    /// A golf ball on a tee, representing golf.
    GolfBallTee,
    /// The logo of Google, a popular search engine.
    Google,
    /// The logo of Google Pay, representing the payment service.
    GooglePay,
    /// The logo of Google Wallet, representing the payment service.
    GoogleWallet,
    /// A gopuram, representing a Hindu temple tower.
    Gopuram,
    /// A graduation cap, representing education or graduation.
    GraduationCap,
    /// A greater than sign, representing mathematical operations.
    GreaterThan,
    /// A greater than or equal sign, representing mathematical operations.
    GreaterThanEqual,
    /// Dots indicating grip or draggable interface.
    Grip,
    /// Horizontal lines indicating grip or draggable interface.
    GripLines,
    /// Vertical lines indicating grip or draggable interface.
    GripLinesVertical,
    /// Vertical dots indicating grip or draggable interface.
    GripVertical,
    /// Multiple arrows rotating around a group, indicating movement or
    /// rotation.
    GroupArrowsRotate,
    /// The symbol for the Paraguayan guaraní, indicating currency.
    GuaraniSign,
    /// A guitar, representing music.
    Guitar,
    /// A gun, representing firearms.
    Gun,
    /// A capital letter 'H', representing the letter.
    H,
    /// A hammer, representing tools or construction.
    Hammer,
    /// A hamsa hand, representing protection or luck.
    Hamsa,
    /// A raised hand, indicating a stop or request for attention.
    Hand,
    /// A hand with a fist facing backwards, representing strength.
    HandBackFist,
    /// A hand with dots, representing tactile or touch.
    HandDots,
    /// A fist, representing strength or power.
    HandFist,
    /// A hand holding something, representing support.
    HandHolding,
    /// A hand holding a dollar sign, representing financial support.
    HandHoldingDollar,
    /// A hand holding a droplet, representing water or liquid.
    HandHoldingDroplet,
    /// A hand holding another hand, representing support or assistance.
    HandHoldingHand,
    /// A hand holding a heart, symbolizing charity or care.
    HandHoldingHeart,
    /// A hand holding a medical symbol, representing healthcare or support.
    HandHoldingMedical,
    /// A hand making a lizard gesture, representing the animal.
    HandLizard,
    /// A hand making the middle finger gesture, representing rudeness.
    HandMiddleFinger,
    /// A hand making a peace sign, representing peace or victory.
    HandPeace,
    /// A hand pointing down, representing direction.
    HandPointDown,
    /// A hand pointing to the left, representing direction.
    HandPointLeft,
    /// A hand pointing to the right, representing direction.
    HandPointRight,
    /// A hand pointing upwards, indicating direction or emphasis.
    HandPointUp,
    /// A hand pointer, representing selection or clicking.
    HandPointer,
    /// A hand making a scissors gesture, representing the game
    /// rock-paper-scissors.
    HandScissors,
    /// A hand with sparkles, representing magic or cleanliness.
    HandSparkles,
    /// A hand making the Vulcan salute, representing Star Trek.
    HandSpock,
    /// A pair of handcuffs, representing law enforcement or restraint.
    Handcuffs,
    /// Two hands, indicating help or collaboration.
    Hands,
    /// Hands signing in ASL, representing sign language.
    HandsAslInterpreting,
    /// A pair of hands bound together, representing restraint or solidarity.
    HandsBound,
    /// Hands with bubbles, representing washing or cleanliness.
    HandsBubbles,
    /// Hands clapping, representing applause or appreciation.
    HandsClapping,
    /// A pair of hands holding something, representing support or unity.
    HandsHolding,
    /// A pair of hands holding a child, representing care or protection.
    HandsHoldingChild,
    /// A pair of hands holding a circle, representing support or unity.
    HandsHoldingCircle,
    /// Hands in a praying position, representing prayer or hope.
    HandsPraying,
    /// Two hands shaking, indicating agreement or partnership.
    Handshake,
    /// A handshake at an angle, representing agreement or partnership.
    HandshakeAngle,
    /// A simple handshake, representing agreement or partnership.
    HandshakeSimple,
    /// A handshake with a slash, indicating no agreement.
    HandshakeSimpleSlash,
    /// Two hands shaking with a slash through them, indicating no agreement or
    /// social distancing.
    HandshakeSlash,
    /// A Hanukkah menorah, representing the Jewish festival.
    Hanukiah,
    /// A hard drive, representing computer storage.
    HardDrive,
    /// A hashtag symbol, representing social media or categorization.
    Hashtag,
    /// A cowboy hat, representing Western style.
    HatCowboy,
    /// A cowboy hat from the side, representing Western style.
    HatCowboySide,
    /// A wizard hat, representing magic or fantasy.
    HatWizard,
    /// A head coughing, representing illness or discomfort.
    HeadSideCough,
    /// A head with a slash, indicating no coughing.
    HeadSideCoughSlash,
    /// A head with a mask, representing health protection.
    HeadSideMask,
    /// A head with a virus, representing infection or illness.
    HeadSideVirus,
    /// A large capital letter 'A', representing text heading.
    Heading,
    /// A pair of headphones, indicating audio or music listening.
    Headphones,
    /// Simple headphones, representing audio listening.
    HeadphonesSimple,
    /// A headset, representing audio communication.
    Headset,
    /// A heart shape, symbolizing love or likes.
    Heart,
    /// A heart with a circled bolt, representing energetic love.
    HeartCircleBolt,
    /// A heart with a circled check mark, representing acceptance or love.
    HeartCircleCheck,
    /// A heart inside a circle with an exclamation mark, representing urgent
    /// health.
    HeartCircleExclamation,
    /// A heart inside a circle with a minus sign, representing health
    /// reduction.
    HeartCircleMinus,
    /// A heart inside a circle with a plus sign, representing health or medical
    /// support.
    HeartCirclePlus,
    /// A heart with a circled `X`, representing rejection or loss.
    HeartCircleXmark,
    /// A broken heart, representing heartbreak or sadness.
    HeartCrack,
    /// A heart with a pulse line, representing health or cardiology.
    HeartPulse,
    /// A helicopter, representing aviation.
    Helicopter,
    /// A helicopter in flight, representing aviation.
    HelicopterSymbol,
    /// A safety helmet, representing protection or construction.
    HelmetSafety,
    /// A helmet with 'UN', representing United Nations peacekeepers.
    HelmetUn,
    /// A highlighter pen, representing marking or emphasis.
    Highlighter,
    /// A hill with an avalanche, representing snow slides.
    HillAvalanche,
    /// A hill with a rockslide, representing landslides.
    HillRockslide,
    /// An icon of a hippo, representing the animal.
    Hippo,
    /// A hockey puck, representing the sport of hockey.
    HockeyPuck,
    /// A holly berry, representing Christmas or winter.
    HollyBerry,
    /// A horse, representing the animal.
    Horse,
    /// A horse head, representing the animal or chess piece.
    HorseHead,
    /// A hospital building, representing healthcare services.
    Hospital,
    /// A hospital with a user, representing a healthcare facility.
    HospitalUser,
    /// A person in a hot tub, representing relaxation or spa.
    HotTubPerson,
    /// A hotdog, representing fast food.
    Hotdog,
    /// A bed with a person, representing a hotel or accommodation.
    Hotel,
    /// An hourglass, representing time.
    Hourglass,
    /// An hourglass almost empty, representing time running out.
    HourglassEnd,
    /// An hourglass half full, representing time.
    HourglassHalf,
    /// An hourglass with sand at the top, indicating the start of a timer.
    HourglassStart,
    /// A simple outline of a house.
    House,
    /// A house with a chimney, representing a home.
    HouseChimney,
    /// A house with a chimney and crack, representing damage.
    HouseChimneyCrack,
    /// A house with a chimney and medical symbol, representing a home medical
    /// facility.
    HouseChimneyMedical,
    /// A house with a chimney and user, representing a home resident.
    HouseChimneyUser,
    /// A house with a chimney and window, representing a home.
    HouseChimneyWindow,
    /// A house with a circled check mark, representing approval.
    HouseCircleCheck,
    /// A house with a circled exclamation mark, representing caution.
    HouseCircleExclamation,
    /// A house with a circled `X`, representing exclusion.
    HouseCircleXmark,
    /// A house with a crack, representing damage or earthquake.
    HouseCrack,
    /// A house with flames, representing a fire emergency.
    HouseFire,
    /// A house with a flag, representing home pride.
    HouseFlag,
    /// A house with water, representing flooding.
    HouseFloodWater,
    /// A house with water and an arrow, representing flood direction.
    HouseFloodWaterCircleArrowRight,
    /// A house with a laptop, representing remote work or home office.
    HouseLaptop,
    /// A house with a lock, representing home security.
    HouseLock,
    /// A house with a medical symbol, representing a medical facility.
    HouseMedical,
    /// A house with a medical symbol and a check mark, representing an approved
    /// medical facility.
    HouseMedicalCircleCheck,
    /// A house with a medical symbol and an exclamation mark, representing a
    /// medical facility with caution.
    HouseMedicalCircleExclamation,
    /// A house with a medical symbol and an `X`, representing a medical
    /// facility with restriction.
    HouseMedicalCircleXmark,
    /// A house with a medical flag, representing a medical facility.
    HouseMedicalFlag,
    /// A house with a signal, representing smart home or connectivity.
    HouseSignal,
    /// A house with a tsunami wave, representing natural disaster.
    HouseTsunami,
    /// A house with a user inside, representing home or resident.
    HouseUser,
    /// The symbol for the Ukrainian hryvnia, indicating currency.
    HryvniaSign,
    /// A hurricane symbol, representing severe weather.
    Hurricane,
    /// The letter \"I\", representing the alphabet.
    I,
    /// An I-beam cursor, representing text selection.
    ICursor,
    /// An ice cream cone, representing dessert or treats.
    IceCream,
    /// Ice formations, representing cold or winter.
    Icicles,
    /// A collection of small icons, representing various symbols.
    Icons,
    /// An ID badge, representing identification.
    IdBadge,
    /// An ID card, representing identification.
    IdCard,
    /// An ID card with a clip, representing identification.
    IdCardClip,
    /// An igloo, representing an Inuit house or cold regions.
    Igloo,
    /// A picture or photo icon, representing image content.
    Image,
    /// A portrait image, representing photos or profiles.
    ImagePortrait,
    /// Multiple images, representing photo galleries or collections.
    Images,
    /// A tray filled with documents, representing an inbox or received
    /// messages.
    Inbox,
    /// An indented line, representing text formatting.
    Indent,
    /// The symbol for the Indian rupee, indicating currency.
    IndianRupeeSign,
    /// A factory, representing industry or manufacturing.
    Industry,
    /// An infinity symbol, representing limitless or infinite.
    Infinity,
    /// A lowercase 'i' in a circle, indicating information.
    Info,
    /// The logo of Instagram, representing the social media platform.
    Instagram,
    /// An italic 'I', representing italicized text.
    Italic,
    /// A capital letter 'J', representing the letter.
    J,
    /// A simple jar, representing containers or storage.
    Jar,
    /// A jar with wheat, representing food storage.
    JarWheat,
    /// The symbol for the Jedi Order, representing Star Wars or spirituality.
    Jedi,
    /// The logo of Jedi Order, representing the Star Wars faction.
    JediOrder,
    /// A jet fighter, representing aviation or military.
    JetFighter,
    /// A jet fighter pointing up, representing aviation or military.
    JetFighterUp,
    /// Two linked circles, representing a connection or joint.
    Joint,
    /// A jug of detergent, representing cleaning supplies.
    JugDetergent,
    /// A capital letter 'K', representing the letter.
    K,
    /// The Kaaba, representing the holy site in Islam.
    Kaaba,
    /// A key, representing access or security.
    Key,
    /// A keyboard, representing typing or computing.
    Keyboard,
    /// The symbol for Khanda, representing Sikhism.
    Khanda,
    /// The logo of Kickstarter, a crowdfunding platform.
    Kickstarter,
    /// The symbol for the Lao kip, indicating currency.
    KipSign,
    /// A medical kit, representing health supplies.
    KitMedical,
    /// Kitchen utensils, representing cooking or kitchen.
    KitchenSet,
    /// A kiwi bird, representing the bird or New Zealand.
    KiwiBird,
    /// A capital letter 'L', representing the letter.
    L,
    /// A land mine, representing explosive devices.
    LandMineOn,
    /// A landmark, representing important places or structures.
    Landmark,
    /// A landmark with a dome, representing a notable building.
    LandmarkDome,
    /// A landmark with a flag, representing a notable location.
    LandmarkFlag,
    /// A globe with characters, indicating language or translation.
    Language,
    /// A laptop computer, representing computing or work.
    Laptop,
    /// A laptop with code, representing programming or development.
    LaptopCode,
    /// A laptop with a file, representing digital documents.
    LaptopFile,
    /// A laptop with a medical symbol, representing telehealth or medical
    /// records.
    LaptopMedical,
    /// The symbol for the Georgian lari, indicating currency.
    LariSign,
    /// Three stacked layers, indicating layering or grouping.
    LayerGroup,
    /// A leaf, representing nature or eco-friendliness.
    Leaf,
    /// A long arrow pointing left, indicating direction.
    LeftLong,
    /// An arrow pointing left and right, indicating bidirectional movement.
    LeftRight,
    /// A lemon fruit, indicating the fruit or something sour.
    Lemon,
    /// A less than sign, representing mathematical operations.
    LessThan,
    /// A less than or equal sign, representing mathematical operations.
    LessThanEqual,
    /// A life ring, representing safety or rescue.
    LifeRing,
    /// A lightbulb, representing ideas or illumination.
    Lightbulb,
    /// Leaning lines, representing design or structure.
    LinesLeaning,
    /// A chain link, indicating a hyperlink or connection.
    Link,
    /// A broken link, representing a disconnected hyperlink.
    LinkSlash,
    /// The logo of `LinkedIn`, representing the professional networking site.
    Linkedin,
    /// The symbol for the Turkish lira, indicating currency.
    LiraSign,
    /// A simple list, representing items or data.
    List,
    /// A list with check marks, representing tasks or to-do lists.
    ListCheck,
    /// An ordered list, representing a sequence or ranking.
    ListOl,
    /// A list with bullet points, representing unordered lists.
    ListUl,
    /// The symbol for Litecoin, indicating cryptocurrency.
    LitecoinSign,
    /// An arrow on a map, representing direction.
    LocationArrow,
    /// A crosshairs on a map, representing target location.
    LocationCrosshairs,
    /// A pinpoint marker, indicating a location on a map.
    LocationDot,
    /// A pin marker, indicating a specific location.
    LocationPin,
    /// A location pin with a lock, representing secure location.
    LocationPinLock,
    /// A padlock, representing security or privacy.
    Lock,
    /// A padlock that is open, representing access or security.
    LockOpen,
    /// A locust, representing the insect or a plague.
    Locust,
    /// A pair of lungs, representing the respiratory system.
    Lungs,
    /// A pair of lungs with a virus, representing respiratory illness.
    LungsVirus,
    /// A capital letter 'M', representing the letter.
    M,
    /// A horseshoe magnet, representing attraction or magnetic fields.
    Magnet,
    /// A magnifying glass, often used to represent search functionality.
    MagnifyingGlass,
    /// A magnifying glass with a right arrow, representing search direction.
    MagnifyingGlassArrowRight,
    /// A magnifying glass with a chart, representing detailed analysis.
    MagnifyingGlassChart,
    /// A magnifying glass with a dollar sign, representing financial search.
    MagnifyingGlassDollar,
    /// A magnifying glass over a location pin, representing search location.
    MagnifyingGlassLocation,
    /// A magnifying glass with a minus sign, representing zoom out or search.
    MagnifyingGlassMinus,
    /// A magnifying glass with a plus sign, representing zoom in or search.
    MagnifyingGlassPlus,
    /// The symbol for the Azerbaijani manat, indicating currency.
    ManatSign,
    /// A folded map, representing navigation or geography.
    Map,
    /// A map with a pin, representing location or navigation.
    MapLocation,
    /// A map pin with a dot, representing location or navigation.
    MapLocationDot,
    /// A map pin, representing location or navigation.
    MapPin,
    /// A marker, representing writing or drawing tools.
    Marker,
    /// The symbol for Mars, representing the planet or male gender.
    Mars,
    /// The symbols for Mars and Venus, representing gender or relationships.
    MarsAndVenus,
    /// The symbols for Mars and Venus with a burst, indicating gender
    /// diversity.
    MarsAndVenusBurst,
    /// Two Mars symbols, representing male gender or masculinity.
    MarsDouble,
    /// The Mars stroke symbol, representing a variation of the male gender
    /// symbol.
    MarsStroke,
    /// The symbol for Mars with a right arrow, indicating male gender or
    /// masculinity.
    MarsStrokeRight,
    /// The symbol for Mars with an upward arrow, indicating male gender or
    /// masculinity.
    MarsStrokeUp,
    /// A martini glass, representing beverages or cocktails.
    MartiniGlass,
    /// A martini glass with a citrus slice, representing beverages or
    /// cocktails.
    MartiniGlassCitrus,
    /// An empty martini glass, representing beverages or cocktails.
    MartiniGlassEmpty,
    /// A theater mask, representing performance or disguise.
    Mask,
    /// A face mask, representing health or safety.
    MaskFace,
    /// A medical mask, representing health protection.
    MaskVentilator,
    /// Two theater masks, representing performance or drama.
    MasksTheater,
    /// A mattress with a pillow, representing bedding or sleep.
    MattressPillow,
    /// A square with arrows pointing outwards, indicating maximization.
    Maximize,
    /// A medal, representing achievement or award.
    Medal,
    /// The logo of Medium, a publishing platform.
    Medium,
    /// A microchip, representing memory or computing hardware.
    Memory,
    /// A menorah, representing the Jewish candelabrum.
    Menorah,
    /// The symbol for the planet Mercury, representing the celestial body or
    /// the element.
    Mercury,
    /// A speech bubble, representing communication or messaging.
    Message,
    /// A meteor, representing space or celestial events.
    Meteor,
    /// A microchip, representing technology or computing.
    Microchip,
    /// A microphone, representing audio or recording.
    Microphone,
    /// A microphone with lines, representing audio recording or broadcasting.
    MicrophoneLines,
    /// A microphone with a slash, indicating no audio recording.
    MicrophoneLinesSlash,
    /// A microphone with a slash, indicating mute or no sound.
    MicrophoneSlash,
    /// A microscope, representing science or research.
    Microscope,
    /// A sign for mills, representing currency or measurement.
    MillSign,
    /// A minimized window, representing reduction.
    Minimize,
    /// A minus sign, indicating subtraction or decrease.
    Minus,
    /// A mitten, representing winter clothing.
    Mitten,
    /// A mobile phone, indicating communication or devices.
    Mobile,
    /// A mobile phone with buttons, representing old-style mobile device.
    MobileButton,
    /// A retro mobile phone, representing old technology.
    MobileRetro,
    /// A mobile phone, representing mobile device.
    MobileScreen,
    /// A mobile phone with a button, representing mobile device.
    MobileScreenButton,
    /// A paper bill, representing money or currency.
    MoneyBill,
    /// A money bill, representing payment or currency.
    MoneyBill1,
    /// A money bill with a wave, representing payment or transaction.
    MoneyBill1Wave,
    /// A money bill with an arrow, indicating financial transfer.
    MoneyBillTransfer,
    /// A money bill with an upward trend, representing financial growth.
    MoneyBillTrendUp,
    /// A waving money bill, representing cash flow.
    MoneyBillWave,
    /// A money bill with wheat, representing agricultural subsidy or trade.
    MoneyBillWheat,
    /// A stack of money bills, representing wealth or currency.
    MoneyBills,
    /// A check, representing financial transactions.
    MoneyCheck,
    /// A check with a dollar sign, representing financial transactions.
    MoneyCheckDollar,
    /// A monument, representing historical or cultural significance.
    Monument,
    /// A crescent moon, representing night or sleep mode.
    Moon,
    /// A mortar and pestle, representing grinding or pharmacy.
    MortarPestle,
    /// A mosque, representing Islamic place of worship.
    Mosque,
    /// A mosquito, representing the insect or disease vector.
    Mosquito,
    /// A mosquito net, representing protection from insects.
    MosquitoNet,
    /// A motorcycle, representing motorbiking.
    Motorcycle,
    /// A mound of earth, representing a small hill or pile.
    Mound,
    /// A mountain, representing nature or hiking.
    Mountain,
    /// A cityscape with mountains, representing urban and natural landscape.
    MountainCity,
    /// A mountain with a sun, indicating landscape or outdoors.
    MountainSun,
    /// A hot mug, representing a hot beverage.
    MugHot,
    /// A mug on a saucer, representing coffee or tea.
    MugSaucer,
    /// A musical note, representing music or audio.
    Music,
    /// A capital letter 'N', representing the letter.
    N,
    /// The symbol for the Nigerian naira, indicating currency.
    NairaSign,
    /// The logo of Napster, representing the music streaming service.
    Napster,
    /// A network of connected nodes, representing wired networking.
    NetworkWired,
    /// The gender symbol for neuter, indicating neutrality.
    Neuter,
    /// A newspaper, indicating news or publications.
    Newspaper,
    /// The logo of NFC Directional, representing near-field communication.
    NfcDirectional,
    /// The NFC (Near Field Communication) symbol, representing wireless
    /// communication.
    NfcSymbol,
    /// A not equal sign, indicating inequality or difference.
    NotEqual,
    /// The .notdef glyph, representing missing characters in typography.
    Notdef,
    /// A sticky note, representing reminders or notes.
    NoteSticky,
    /// A clipboard with medical notes, representing healthcare documentation.
    NotesMedical,
    /// A capital letter 'O', representing the letter or shape.
    O,
    /// An icon of grouped objects, indicating grouping.
    ObjectGroup,
    /// An icon of separated objects, indicating ungrouping.
    ObjectUngroup,
    /// An oil can, representing lubrication or mechanics.
    OilCan,
    /// An oil well, representing fossil fuels or drilling.
    OilWell,
    /// The logo of Old Republic, representing the Star Wars faction.
    OldRepublic,
    /// The Om symbol, representing Hinduism.
    Om,
    /// An otter, representing the animal.
    Otter,
    /// Text with a reduced indent, representing text alignment.
    Outdent,
    /// A capital letter 'P', representing the letter or parking.
    P,
    /// A pager, representing communication devices.
    Pager,
    /// A paint roller, indicating painting or renovation.
    PaintRoller,
    /// A paintbrush, representing painting or art.
    Paintbrush,
    /// A painter's palette, representing art or color selection.
    Palette,
    /// A pallet, representing shipping or logistics.
    Pallet,
    /// A wide-angle view, representing landscape photography.
    Panorama,
    /// A paper plane, indicating sending a message or flying.
    PaperPlane,
    /// A paperclip, representing attachment or link.
    Paperclip,
    /// A box with a parachute, representing delivery or drop.
    ParachuteBox,
    /// A paragraph symbol, representing text.
    Paragraph,
    /// A passport, representing international travel.
    Passport,
    /// A clipboard with a document, representing pasting.
    Paste,
    /// A pause symbol, representing media pause.
    Pause,
    /// A paw print, representing animals or pets.
    Paw,
    /// The logo of `PayPal`, an online payment system.
    Paypal,
    /// A peace symbol, representing peace or anti-war.
    Peace,
    /// A pen, representing writing or creativity.
    Pen,
    /// A pen with a clip, representing writing or stationery.
    PenClip,
    /// A fancy pen, representing writing or creativity.
    PenFancy,
    /// An old-fashioned pen nib, representing writing or creativity.
    PenNib,
    /// A pen and ruler, representing drawing or design.
    PenRuler,
    /// A pen writing on a square, indicating editing or writing.
    PenToSquare,
    /// A pencil, representing writing or editing.
    Pencil,
    /// Two people with arrows pointing towards each other, representing
    /// communication or interaction.
    PeopleArrows,
    /// People carrying a box, representing moving or teamwork.
    PeopleCarryBox,
    /// Multiple people, representing a group or community.
    PeopleGroup,
    /// People standing in line, representing queue.
    PeopleLine,
    /// Two people pulling, representing teamwork or effort.
    PeoplePulling,
    /// A person being robbed, representing crime or danger.
    PeopleRobbery,
    /// People under a roof, representing shelter or protection.
    PeopleRoof,
    /// A hot pepper, representing spicy food.
    PepperHot,
    /// A percent sign, indicating percentages or discounts.
    Percent,
    /// A person, representing an individual or user.
    Person,
    /// A person with an arrow pointing down to a line, indicating descending or
    /// moving down.
    PersonArrowDownToLine,
    /// A person with an arrow pointing up from a line, indicating rising or
    /// moving up.
    PersonArrowUpFromLine,
    /// A person biking, representing cycling.
    PersonBiking,
    /// A person in a booth, indicating privacy or voting.
    PersonBooth,
    /// A person breastfeeding, representing motherhood or childcare.
    PersonBreastfeeding,
    /// A person with a burst, indicating excitement or energy.
    PersonBurst,
    /// A person with a cane, representing disability or assistance.
    PersonCane,
    /// A person at a chalkboard, representing teaching or presentation.
    PersonChalkboard,
    /// A person inside a circle with a check mark, representing verification.
    PersonCircleCheck,
    /// A person with a circled exclamation mark, indicating warning.
    PersonCircleExclamation,
    /// A person with a circled minus, indicating removal or exclusion.
    PersonCircleMinus,
    /// A person with a circled plus, indicating addition or inclusion.
    PersonCirclePlus,
    /// A person with a circled question mark, indicating inquiry or
    /// uncertainty.
    PersonCircleQuestion,
    /// A person with a circled `X`, indicating exclusion.
    PersonCircleXmark,
    /// A person digging, representing construction or excavation.
    PersonDigging,
    /// A person with dots moving from a line, representing transition or
    /// movement.
    PersonDotsFromLine,
    /// A person wearing a dress, representing clothing or fashion.
    PersonDress,
    /// A person in a dress with a burst, indicating excitement or motion.
    PersonDressBurst,
    /// A person drowning, representing danger in water.
    PersonDrowning,
    /// A person falling, representing accident or failure.
    PersonFalling,
    /// A person falling with a burst, representing injury or accident.
    PersonFallingBurst,
    /// A person wearing half a dress, representing fashion or gender fluidity.
    PersonHalfDress,
    /// A person harassing another, representing harassment.
    PersonHarassing,
    /// A person hiking, representing outdoor activities.
    PersonHiking,
    /// A military person pointing, indicating direction or command.
    PersonMilitaryPointing,
    /// A military person holding a rifle, representing armed forces.
    PersonMilitaryRifle,
    /// A military person saluting another person, representing respect.
    PersonMilitaryToPerson,
    /// A person praying, representing spirituality or religion.
    PersonPraying,
    /// A pregnant person, representing pregnancy.
    PersonPregnant,
    /// A person with rays, representing radiance or positivity.
    PersonRays,
    /// A person holding a rifle, representing shooting sports or military.
    PersonRifle,
    /// A person running, representing movement or exercise.
    PersonRunning,
    /// A person under a shelter, representing protection or safety.
    PersonShelter,
    /// A person skating, representing the sport or activity.
    PersonSkating,
    /// A person skiing, representing winter sports.
    PersonSkiing,
    /// A person skiing Nordic style, representing skiing.
    PersonSkiingNordic,
    /// A person snowboarding, representing winter sports.
    PersonSnowboarding,
    /// A person swimming, representing swimming or water sports.
    PersonSwimming,
    /// A person moving through a window, indicating escape or emergency exit.
    PersonThroughWindow,
    /// A person walking, representing movement.
    PersonWalking,
    /// A person walking with a looping arrow to the left, indicating return or
    /// reverse.
    PersonWalkingArrowLoopLeft,
    /// A person walking with an arrow, indicating movement.
    PersonWalkingArrowRight,
    /// A person walking with a dashed line and arrow, indicating a guided path.
    PersonWalkingDashedLineArrowRight,
    /// A person walking with luggage, indicating travel.
    PersonWalkingLuggage,
    /// A person walking with a cane, indicating disability or assistance.
    PersonWalkingWithCane,
    /// The symbol for the Spanish peseta, indicating currency.
    PesetaSign,
    /// The symbol for the Philippine peso, indicating currency.
    PesoSign,
    /// A phone, representing communication or contact.
    Phone,
    /// A phone flipped, indicating mobile communication.
    PhoneFlip,
    /// A phone with a slash, indicating no calls.
    PhoneSlash,
    /// A phone handset with sound waves, indicating a call or audio settings.
    PhoneVolume,
    /// A strip of photo film, representing photography.
    PhotoFilm,
    /// A piggy bank, representing savings or finance.
    PiggyBank,
    /// A pair of pills, representing medication.
    Pills,
    /// A slice of pizza, representing food or dining.
    PizzaSlice,
    /// A place of worship, indicating religious services.
    PlaceOfWorship,
    /// An airplane, indicating travel or flights.
    Plane,
    /// A plane arriving, indicating air travel arrival.
    PlaneArrival,
    /// A plane with a circled check mark, representing flight confirmation.
    PlaneCircleCheck,
    /// A plane with a circled exclamation mark, indicating travel alert.
    PlaneCircleExclamation,
    /// A plane with a circled `X`, indicating no flying.
    PlaneCircleXmark,
    /// A plane taking off, indicating air travel.
    PlaneDeparture,
    /// A plane with a lock, indicating secure travel.
    PlaneLock,
    /// A plane with a slash, indicating no flying.
    PlaneSlash,
    /// A plane taking off, indicating air travel.
    PlaneUp,
    /// A wilted plant, indicating lack of water or poor health.
    PlantWilt,
    /// A plate with wheat, indicating food or meal.
    PlateWheat,
    /// A play button, indicating media playback.
    Play,
    /// The logo of `PlayStation`, a gaming console.
    Playstation,
    /// An electrical plug, indicating power or connectivity.
    Plug,
    /// A plug with a circled bolt, indicating powered connection.
    PlugCircleBolt,
    /// A plug with a circled check, indicating secure connection.
    PlugCircleCheck,
    /// A plug with a circled exclamation mark, representing power alert.
    PlugCircleExclamation,
    /// A plug with a circled minus sign, representing power reduction.
    PlugCircleMinus,
    /// A plug with a circled plus, indicating connection.
    PlugCirclePlus,
    /// A plug with a circled `X`, indicating no connection.
    PlugCircleXmark,
    /// A cross, representing addition or positivity.
    Plus,
    /// A plus and minus sign, indicating addition and subtraction.
    PlusMinus,
    /// A podcast icon, representing audio broadcasting.
    Podcast,
    /// A pile of poo with eyes, often used humorously.
    Poo,
    /// A storm cloud with a poo, often used humorously.
    PooStorm,
    /// A pile of poop, representing waste or humor.
    Poop,
    /// A power button, indicating shutdown or turning off.
    PowerOff,
    /// A prescription symbol, indicating medical prescription.
    Prescription,
    /// A prescription bottle, representing medicine or healthcare.
    PrescriptionBottle,
    /// A medical prescription bottle, indicating medication.
    PrescriptionBottleMedical,
    /// A printer, representing printing documents.
    Print,
    /// A medical pump, indicating medical equipment.
    PumpMedical,
    /// A soap dispenser, representing hygiene or cleanliness.
    PumpSoap,
    /// A puzzle piece, indicating a part of a puzzle.
    PuzzlePiece,
    /// The letter \"Q\", representing the alphabet.
    Q,
    /// A QR code, representing quick response codes for scanning.
    Qrcode,
    /// A question mark, indicating inquiry or help.
    Question,
    /// A left-leaning quotation mark, indicating the start of a quote.
    QuoteLeft,
    /// A right-leaning quotation mark, indicating the end of a quote.
    QuoteRight,
    /// A capital letter 'R', representing the letter or registered trademark.
    R,
    /// A radiation symbol, indicating hazardous materials.
    Radiation,
    /// A radio, representing broadcasting or communication.
    Radio,
    /// A rainbow, representing LGBTQ+ pride or spectrum.
    Rainbow,
    /// A star with a number, indicating rank or rating.
    RankingStar,
    /// A receipt, representing a transaction record.
    Receipt,
    /// A vinyl record, representing music or audio.
    RecordVinyl,
    /// A rectangle with 'AD' inside, indicating advertisement.
    RectangleAd,
    /// A rectangle with a list inside, representing menu or options.
    RectangleList,
    /// A rectangle with an `X`, indicating deletion or closure.
    RectangleXmark,
    /// Three arrows forming a triangle, indicating recycling.
    Recycle,
    /// A circled 'R', indicating a registered trademark.
    Registered,
    /// Two arrows forming a circle, indicating repeat or refresh.
    Repeat,
    /// An arrow pointing left, indicating a reply.
    Reply,
    /// A reply-all symbol, representing email or messaging.
    ReplyAll,
    /// An elephant, representing the Republican party.
    Republican,
    /// A man and woman icon, indicating restroom facilities.
    Restroom,
    /// Two arrows forming a square, indicating retweet or repost.
    Retweet,
    /// A ribbon, representing awareness or decoration.
    Ribbon,
    /// An arrow pointing right from a bracket, indicating exit or move.
    RightFromBracket,
    /// An arrow pointing right and left, indicating bidirectional movement.
    RightLeft,
    /// A long arrow pointing right, indicating forward direction.
    RightLong,
    /// An arrow pointing right into a bracket, indicating entering or logging
    /// in.
    RightToBracket,
    /// A ring, representing jewelry or engagement.
    Ring,
    /// A road, indicating travel or transportation.
    Road,
    /// A road barrier, indicating roadblock or construction.
    RoadBarrier,
    /// A bridge, representing transportation infrastructure.
    RoadBridge,
    /// A road with a circled check mark, representing approved routes.
    RoadCircleCheck,
    /// A road with a circled exclamation mark, indicating caution or warning.
    RoadCircleExclamation,
    /// A road with a circled `X`, indicating road closure.
    RoadCircleXmark,
    /// A road with a lock, indicating restricted access.
    RoadLock,
    /// Spikes on the road, representing security or vehicle stop.
    RoadSpikes,
    /// A robot, representing automation or robotics.
    Robot,
    /// A rocket, indicating space exploration or rapid progress.
    Rocket,
    /// A circular arrow, indicating rotation or refresh.
    Rotate,
    /// An arrow rotating to the left, indicating undo or backward.
    RotateLeft,
    /// An arrow rotating to the right, indicating redo or refresh.
    RotateRight,
    /// A winding road, indicating a path or journey.
    Route,
    /// A feed icon, representing RSS feed.
    Rss,
    /// The symbol for the Russian ruble, indicating currency.
    RubleSign,
    /// A rug, representing home decor or carpeting.
    Rug,
    /// A ruler, representing measurement.
    Ruler,
    /// A ruler combined with another tool, representing measurement.
    RulerCombined,
    /// A horizontal ruler, representing measurement.
    RulerHorizontal,
    /// A vertical ruler, representing measurement.
    RulerVertical,
    /// The symbol for the Indian rupee, indicating currency.
    RupeeSign,
    /// The symbol for the Indonesian rupiah, indicating currency.
    RupiahSign,
    /// A capital letter 'S', representing the letter or Superman.
    S,
    /// A sack with a dollar sign, indicating money or wealth.
    SackDollar,
    /// A sack with an `X`, indicating no contents or emptiness.
    SackXmark,
    /// A sailboat, representing sailing or maritime activities.
    Sailboat,
    /// A satellite, representing space or communication.
    Satellite,
    /// A satellite dish, representing communication.
    SatelliteDish,
    /// A balanced scale, representing justice or equality.
    ScaleBalanced,
    /// A tilted scale, indicating imbalance.
    ScaleUnbalanced,
    /// A tilted scale, indicating imbalance.
    ScaleUnbalancedFlip,
    /// A school building, representing education.
    School,
    /// A school building with a check mark in a circle, indicating school
    /// approval.
    SchoolCircleCheck,
    /// A school with a circled exclamation mark, representing school alert.
    SchoolCircleExclamation,
    /// A school with a circled X, representing school closure or cancellation.
    SchoolCircleXmark,
    /// A school building with a flag, representing education or school pride.
    SchoolFlag,
    /// A school building with a lock, indicating school security.
    SchoolLock,
    /// A pair of scissors, representing cutting or crafting.
    Scissors,
    /// A screwdriver, representing tools or repair.
    Screwdriver,
    /// A screwdriver and wrench crossed, representing tools or repair.
    ScrewdriverWrench,
    /// A scroll, representing a document or parchment.
    Scroll,
    /// A scroll, representing the Torah or ancient texts.
    ScrollTorah,
    /// An SD card, representing storage or memory.
    SdCard,
    /// A divided section, representing a part or segment.
    Section,
    /// A small plant sprouting, representing growth or new beginnings.
    Seedling,
    /// A server, representing data storage or hosting.
    Server,
    /// A collection of geometric shapes, representing design or layout.
    Shapes,
    /// An arrow pointing outwards, indicating sharing content.
    Share,
    /// An arrow coming out of a square, indicating sharing or exporting.
    ShareFromSquare,
    /// Three connected nodes, representing sharing or networking.
    ShareNodes,
    /// A sheet of plastic, representing material.
    SheetPlastic,
    /// The symbol for the Israeli shekel, indicating currency.
    ShekelSign,
    /// A shield, representing protection or security.
    Shield,
    /// A shield with a cat, representing pet protection.
    ShieldCat,
    /// A shield with a dog, representing pet protection.
    ShieldDog,
    /// A shield split in half, indicating partial protection or security.
    ShieldHalved,
    /// A shield with a heart, representing health protection.
    ShieldHeart,
    /// A shield with a virus, representing antivirus protection.
    ShieldVirus,
    /// A ship, representing maritime transportation.
    Ship,
    /// A t-shirt, indicating clothing.
    Shirt,
    /// Shoe prints, representing footsteps or tracking.
    ShoePrints,
    /// A store front, indicating shopping or retail.
    Shop,
    /// A shop with a lock, representing a closed store.
    ShopLock,
    /// A shop with a slash, indicating closed or no shopping.
    ShopSlash,
    /// The logo of Shopify, an e-commerce platform.
    Shopify,
    /// A shower head with water, indicating bathing.
    Shower,
    /// A shrimp, representing seafood.
    Shrimp,
    /// Two arrows crossing, indicating shuffle or random order.
    Shuffle,
    /// A space shuttle, representing space exploration.
    ShuttleSpace,
    /// A hanging sign, representing a signboard or notice.
    SignHanging,
    /// A signal tower with waves, representing communication or connectivity.
    Signal,
    /// A handwritten signature, indicating signing or approval.
    Signature,
    /// A signpost, representing directions or navigation.
    SignsPost,
    /// A SIM card, representing mobile connectivity.
    SimCard,
    /// A sink, representing kitchen or bathroom fixtures.
    Sink,
    /// A hierarchical diagram, representing a sitemap or organization chart.
    Sitemap,
    /// A simple skull, representing death or danger.
    Skull,
    /// A skull with crossbones, representing danger or pirates.
    SkullCrossbones,
    /// The logo of Slack, a communication platform for teams.
    Slack,
    /// A slash symbol, representing separation or division.
    Slash,
    /// A sleigh, representing Christmas or winter transport.
    Sleigh,
    /// Sliders, representing controls or adjustments.
    Sliders,
    /// A city skyline with smog, representing air pollution.
    Smog,
    /// A cigarette with smoke, representing smoking.
    Smoking,
    /// A snowflake, representing cold or winter.
    Snowflake,
    /// A snowman, representing winter or Christmas.
    Snowman,
    /// A snowplow vehicle, representing snow removal.
    Snowplow,
    /// A bar of soap, representing cleanliness or hygiene.
    Soap,
    /// A pair of socks, representing clothing.
    Socks,
    /// A solar panel, representing solar energy.
    SolarPanel,
    /// Three stacked horizontal lines, indicating sorting.
    Sort,
    /// A list with a downward arrow, indicating sorting in descending order.
    SortDown,
    /// A list with an upward arrow, indicating sorting in ascending order.
    SortUp,
    /// The logo of `SoundCloud`, representing the music platform.
    Soundcloud,
    /// A flower with petals, representing relaxation or spa.
    Spa,
    /// The logo of Space Awesome, representing the brand or company.
    SpaceAwesome,
    /// A flying spaghetti monster, representing parody religion.
    SpaghettiMonsterFlying,
    /// A check mark with ABC, representing spell checking.
    SpellCheck,
    /// A spider, representing the arachnid or Halloween.
    Spider,
    /// A spinning circle, indicating loading or processing.
    Spinner,
    /// A paint splotch, representing color or mess.
    Splotch,
    /// A spoon, representing dining or kitchen utensils.
    Spoon,
    /// The logo of Spotify, a music streaming service.
    Spotify,
    /// A spray can, representing painting or spraying.
    SprayCan,
    /// A spray can emitting sparkles, representing spray effects.
    SprayCanSparkles,
    /// A simple square, representing shape or stop.
    Square,
    /// A square with an arrow pointing up and right, indicating expansion or
    /// exit.
    SquareArrowUpRight,
    /// A square with a downward caret, representing more options or dropdowns.
    SquareCaretDown,
    /// A square with a leftward caret, representing navigation or more options.
    SquareCaretLeft,
    /// A square with a rightward caret, representing navigation or more
    /// options.
    SquareCaretRight,
    /// A square with an upward caret, representing navigation or more options.
    SquareCaretUp,
    /// A square with a check mark, indicating completion or approval.
    SquareCheck,
    /// A square with an envelope, representing mail or messages.
    SquareEnvelope,
    /// A square completely filled, representing fullness or completeness.
    SquareFull,
    /// A square with an 'H', representing hospital.
    SquareH,
    /// A square with a minus sign, indicating removal or decrease.
    SquareMinus,
    /// A square with 'NFI', indicating an undefined acronym.
    SquareNfi,
    /// A square with a 'P', representing parking.
    SquareParking,
    /// A square with a pen, representing editing or writing.
    SquarePen,
    /// A square with a person confined inside, representing isolation.
    SquarePersonConfined,
    /// A square with a phone icon, representing communication or device.
    SquarePhone,
    /// A square with a phone icon flipped, indicating phone rotation.
    SquarePhoneFlip,
    /// A square with a plus sign, indicating addition or increase.
    SquarePlus,
    /// A square with horizontal bars, representing a horizontal poll or chart.
    SquarePollHorizontal,
    /// A square with vertical bars, representing a vertical poll or chart.
    SquarePollVertical,
    /// A square with a variable inside a root symbol, representing mathematics.
    SquareRootVariable,
    /// A square with RSS icon, representing news feed.
    SquareRss,
    /// A square with nodes connected by lines, indicating sharing or
    /// networking.
    SquareShareNodes,
    /// A square with the Steam logo, representing the gaming platform.
    SquareSteam,
    /// A square with an arrow pointing up and right, indicating expansion or
    /// exit.
    SquareUpRight,
    /// A square with virus icons, representing illness or infection.
    SquareVirus,
    /// A square with an `X`, representing rejection or closure.
    SquareXmark,
    /// The logo of Squarespace, a website building platform.
    Squarespace,
    /// The logo of Stack Overflow, a Q&A platform for developers.
    StackOverflow,
    /// A staff with a snake, representing medical profession.
    StaffSnake,
    /// A staircase, representing steps or levels.
    Stairs,
    /// A stamp, representing approval or postage.
    Stamp,
    /// A stapler, representing office supplies.
    Stapler,
    /// A star, often used to represent favorites or ratings.
    Star,
    /// A star and crescent, representing Islam.
    StarAndCrescent,
    /// A half-filled star, indicating partial rating.
    StarHalf,
    /// A half-filled star, indicating partial rating.
    StarHalfStroke,
    /// A star of David, representing Judaism.
    StarOfDavid,
    /// A six-pointed star with a rod in the center, representing emergency
    /// medical services.
    StarOfLife,
    /// The logo of Steam, representing the gaming platform.
    Steam,
    /// The logo of Steam, representing the gaming platform.
    SteamSymbol,
    /// The symbol for the British pound, indicating currency.
    SterlingSign,
    /// A stethoscope, representing medical examination or healthcare.
    Stethoscope,
    /// A stop sign, indicating cessation or pause.
    Stop,
    /// A simple stopwatch, representing timing.
    Stopwatch,
    /// A stopwatch showing 20 seconds, representing time measurement.
    Stopwatch20,
    /// A storefront, representing retail or shops.
    Store,
    /// A store with a slash, indicating closed or no store.
    StoreSlash,
    /// A street view symbol, representing navigation or mapping.
    StreetView,
    /// Text with a line through it, indicating deletion or correction.
    Strikethrough,
    /// The logo of Stripe, a payment processing platform.
    Stripe,
    /// The logo of Stripe, representing the payment processing platform.
    StripeS,
    /// A stroopwafel, representing the Dutch treat.
    Stroopwafel,
    /// A subscript 'A', indicating subscript text.
    Subscript,
    /// A simple suitcase, representing travel or luggage.
    Suitcase,
    /// A medical suitcase, representing emergency medical kit.
    SuitcaseMedical,
    /// A suitcase with wheels, representing travel.
    SuitcaseRolling,
    /// A sun, representing daytime or brightness.
    Sun,
    /// A sun with a wilted plant, indicating drought or plant stress.
    SunPlantWilt,
    /// A superscript 'A', indicating superscript text.
    Superscript,
    /// A swatchbook, representing color samples or design.
    Swatchbook,
    /// A synagogue, representing a place of worship for Jews.
    Synagogue,
    /// A syringe, representing medical injections.
    Syringe,
    /// A capital letter 'T', representing the letter.
    T,
    /// A simple table, indicating data or spreadsheet.
    Table,
    /// A table with cells, representing data organization.
    TableCells,
    /// A table with a locked column, indicating fixed data.
    TableCellsColumnLock,
    /// A table with large cells, representing data organization.
    TableCellsLarge,
    /// A table with a locked row, indicating fixed data.
    TableCellsRowLock,
    /// A table with columns, representing data organization.
    TableColumns,
    /// A table with a list, representing data organization.
    TableList,
    /// A table tennis paddle with a ball, representing the sport.
    TableTennisPaddleBall,
    /// A tablet device, representing mobile computing.
    Tablet,
    /// A tablet with a button, representing a touchscreen device.
    TabletButton,
    /// A tablet with a screen and button, representing a digital device.
    TabletScreenButton,
    /// Two pills, representing medication or tablets.
    Tablets,
    /// A digital tachograph, representing vehicle monitoring.
    TachographDigital,
    /// A price tag, indicating labels or pricing.
    Tag,
    /// Multiple tags, representing labels or categories.
    Tags,
    /// A roll of tape, representing adhesive tape.
    Tape,
    /// A simple tarp, representing a cover or protection.
    Tarp,
    /// A tarp with a droplet, representing waterproof covering.
    TarpDroplet,
    /// A taxi cab, representing transportation service.
    Taxi,
    /// A set of teeth, representing dental health.
    Teeth,
    /// An open mouth with teeth, representing dental health or smiling.
    TeethOpen,
    /// A thermometer with a downward arrow, indicating falling temperature.
    TemperatureArrowDown,
    /// A thermometer with an upward arrow, indicating rising temperature.
    TemperatureArrowUp,
    /// A thermometer empty, representing no temperature reading.
    TemperatureEmpty,
    /// A thermometer full, representing very high temperature.
    TemperatureFull,
    /// A thermometer half full, representing moderate temperature.
    TemperatureHalf,
    /// A thermometer with high reading, indicating high temperature.
    TemperatureHigh,
    /// A thermometer with low reading, indicating low temperature.
    TemperatureLow,
    /// A thermometer one-quarter full, representing low temperature.
    TemperatureQuarter,
    /// A thermometer three-quarters full, representing high temperature.
    TemperatureThreeQuarters,
    /// The symbol for the Kazakhstani tenge, indicating currency.
    TengeSign,
    /// A single tent, representing camping or temporary shelter.
    Tent,
    /// A tent with an arrow pointing down to a line, representing a campsite.
    TentArrowDownToLine,
    /// A tent with arrows pointing left and right, indicating horizontal setup.
    TentArrowLeftRight,
    /// A tent with an arrow turning left, indicating directional setup.
    TentArrowTurnLeft,
    /// A tent with arrows pointing down, indicating tent setup.
    TentArrowsDown,
    /// Multiple tents, representing camping or temporary shelter.
    Tents,
    /// A computer terminal, representing command line or coding.
    Terminal,
    /// An icon indicating text height adjustment.
    TextHeight,
    /// Text with a slash, indicating no text.
    TextSlash,
    /// An icon indicating text width adjustment.
    TextWidth,
    /// A thermometer, representing temperature measurement.
    Thermometer,
    /// A thumbs-down gesture, indicating disapproval or dislike.
    ThumbsDown,
    /// A thumbs-up gesture, indicating approval or like.
    ThumbsUp,
    /// A thumbtack, indicating pinned items or locations.
    Thumbtack,
    /// A ticket, representing admission or entry to an event.
    Ticket,
    /// A simple ticket, representing admission or entry.
    TicketSimple,
    /// The logo of `TikTok`, a video-sharing social media platform.
    Tiktok,
    /// A timeline, representing chronological events.
    Timeline,
    /// A switch in the off position, indicating deactivation.
    ToggleOff,
    /// A switch in the 'on' position, indicating activation.
    ToggleOn,
    /// A toilet, representing restrooms.
    Toilet,
    /// A roll of toilet paper, indicating sanitation.
    ToiletPaper,
    /// A toilet paper roll with a slash, indicating no toilet paper.
    ToiletPaperSlash,
    /// A portable toilet, representing outdoor facilities.
    ToiletPortable,
    /// Portable toilets, indicating temporary sanitation facilities.
    ToiletsPortable,
    /// A toolbox, representing tools or repair.
    Toolbox,
    /// A tooth, representing dentistry or oral health.
    Tooth,
    /// A torii gate, representing Japanese culture.
    ToriiGate,
    /// A tornado, representing severe weather.
    Tornado,
    /// A broadcast tower, representing media transmission.
    TowerBroadcast,
    /// A cell tower, representing communication.
    TowerCell,
    /// A tall observation tower, representing sightseeing.
    TowerObservation,
    /// A tractor, representing agriculture or farming.
    Tractor,
    /// A trademark symbol, representing brand or intellectual property.
    Trademark,
    /// A traffic light, representing road signals.
    TrafficLight,
    /// A trailer, representing cargo or transport.
    Trailer,
    /// A train, representing railway transport.
    Train,
    /// A subway train, representing underground transportation.
    TrainSubway,
    /// A tram, representing public transportation.
    TrainTram,
    /// A combined male and female symbol, representing transgender identity.
    Transgender,
    /// A trash can, representing deletion or garbage.
    Trash,
    /// A trash can, representing waste disposal.
    TrashCan,
    /// A tree, representing nature or the environment.
    Tree,
    /// A tree with a cityscape, representing urban nature or parks.
    TreeCity,
    /// A triangle with an exclamation mark, indicating warning or caution.
    TriangleExclamation,
    /// A trophy, representing achievement or awards.
    Trophy,
    /// A trowel, representing construction or gardening.
    Trowel,
    /// A trowel with bricks, representing construction or masonry.
    TrowelBricks,
    /// An icon of a truck, indicating transportation or delivery.
    Truck,
    /// A truck with an arrow pointing right, representing delivery.
    TruckArrowRight,
    /// A truck with a droplet, representing liquid transport.
    TruckDroplet,
    /// A fast-moving truck, indicating quick delivery or shipment.
    TruckFast,
    /// A truck in a field, indicating agricultural transport.
    TruckField,
    /// A truck in a field, indicating agricultural transport.
    TruckFieldUn,
    /// A front view of a truck, indicating transportation or delivery.
    TruckFront,
    /// A medical truck, representing emergency medical transport.
    TruckMedical,
    /// A monster truck, indicating a large, powerful vehicle.
    TruckMonster,
    /// A moving truck, representing relocation or transport.
    TruckMoving,
    /// A pickup truck, representing a vehicle or transportation.
    TruckPickup,
    /// A truck with a plane, representing logistics.
    TruckPlane,
    /// A truck with a ramp, indicating delivery or loading.
    TruckRampBox,
    /// An old-fashioned telephone with a keyboard, indicating teletype.
    Tty,
    /// The symbol for the Turkish lira, indicating currency.
    TurkishLiraSign,
    /// An arrow curving downwards, indicating turning down.
    TurnDown,
    /// An arrow curving upwards, indicating turning up.
    TurnUp,
    /// A television set, representing media or entertainment.
    Tv,
    /// The logo of Twitch, a live streaming platform.
    Twitch,
    /// The logo of Twitter, a well-known social media platform.
    Twitter,
    /// The letter \"U\", representing the alphabet.
    U,
    /// An umbrella, indicating protection from rain or sun.
    Umbrella,
    /// An umbrella on a beach, representing leisure or vacation.
    UmbrellaBeach,
    /// A line below text, indicating underline or emphasis.
    Underline,
    /// A circle with a person inside, representing accessibility.
    UniversalAccess,
    /// An open padlock, indicating access or security.
    Unlock,
    /// An open padlock with a keyhole, indicating access or security.
    UnlockKeyhole,
    /// The logo of Unsplash, representing the photo sharing platform.
    Unsplash,
    /// An arrow pointing up and another pointing down, indicating vertical
    /// movement.
    UpDown,
    /// Arrows pointing in all four directions, indicating omnidirectional
    /// movement.
    UpDownLeftRight,
    /// A long arrow pointing up, indicating upward direction or increase.
    UpLong,
    /// An arrow pointing up right and down left from center, indicating
    /// movement.
    UpRightAndDownLeftFromCenter,
    /// An arrow pointing up and right from a square, indicating expansion or
    /// exit.
    UpRightFromSquare,
    /// An arrow pointing upward from a box, indicating upload.
    Upload,
    /// An outline of a person, indicating a user or profile.
    User,
    /// A user icon wearing an astronaut helmet, representing an astronaut or
    /// space exploration.
    UserAstronaut,
    /// A user icon with a check mark, indicating user approval or verification.
    UserCheck,
    /// A user icon with a clock, indicating user schedule or time management.
    UserClock,
    /// A user icon with a stethoscope, representing a doctor or healthcare
    /// professional.
    UserDoctor,
    /// A user icon with a gear, representing user settings or management.
    UserGear,
    /// A user wearing a graduation cap, indicating education or graduation.
    UserGraduate,
    /// Multiple user icons, indicating a group or community.
    UserGroup,
    /// A user with an injury, representing injury or accident.
    UserInjured,
    /// A large user icon, indicating a prominent user.
    UserLarge,
    /// A large user icon with a slash, representing user removal or
    /// restriction.
    UserLargeSlash,
    /// A user with a lock, representing account security.
    UserLock,
    /// A user icon with a minus sign, indicating user removal.
    UserMinus,
    /// A user icon with a ninja mask, indicating a stealthy or anonymous user.
    UserNinja,
    /// A user with a nurse hat, representing medical staff.
    UserNurse,
    /// A user icon with a pen, representing user editing or writing.
    UserPen,
    /// A user icon with a plus sign, indicating adding a user.
    UserPlus,
    /// A user with a finger over their lips, indicating secrecy or
    /// confidentiality.
    UserSecret,
    /// A user with a shield, representing user protection or security.
    UserShield,
    /// A user icon with a slash, indicating a removed or blocked user.
    UserSlash,
    /// A user with a tag, representing user identification.
    UserTag,
    /// A user icon with a tie, indicating a professional user.
    UserTie,
    /// A user with a circled X, representing user removal.
    UserXmark,
    /// Multiple user icons, representing a group or community.
    Users,
    /// Multiple user icons between lines, indicating collaboration or
    /// communication.
    UsersBetweenLines,
    /// Multiple user icons with a gear, representing user settings or
    /// management.
    UsersGear,
    /// Multiple users in a line, representing a group or queue.
    UsersLine,
    /// Multiple users with rays, representing community or influence.
    UsersRays,
    /// Multiple user icons inside a rectangle, representing a group or
    /// community.
    UsersRectangle,
    /// Multiple user icons with a slash, indicating no users or blocked.
    UsersSlash,
    /// A user icon inside a viewfinder, representing focus on users.
    UsersViewfinder,
    /// A fork and knife, representing dining or food.
    Utensils,
    /// A capital letter 'V', representing the letter.
    V,
    /// A shuttle van, representing transportation.
    VanShuttle,
    /// A vault, representing security or storage.
    Vault,
    /// A square with vector points, representing design or graphics.
    VectorSquare,
    /// The symbol of Venus, representing the female gender.
    Venus,
    /// Two Venus symbols, representing female gender or partnership.
    VenusDouble,
    /// The symbols of Venus and Mars combined, representing gender.
    VenusMars,
    /// A vest, representing clothing.
    Vest,
    /// A vest with patches, representing protective gear.
    VestPatches,
    /// A vial, representing a small container for liquids.
    Vial,
    /// A vial with a circled check mark, representing approved substance.
    VialCircleCheck,
    /// A vial with a virus, representing medical testing.
    VialVirus,
    /// Two laboratory vials, representing testing or experimentation.
    Vials,
    /// A video camera, indicating video content or recording.
    Video,
    /// A video symbol with a slash, indicating no video or disabled video.
    VideoSlash,
    /// A Buddhist temple, representing a place of worship.
    Vihara,
    /// A virus, representing infection or disease.
    Virus,
    /// A representation of the COVID-19 virus.
    VirusCovid,
    /// A virus symbol with a slash, representing COVID-19 eradication.
    VirusCovidSlash,
    /// A virus with a slash, indicating antivirus or no virus.
    VirusSlash,
    /// Multiple virus icons, representing infections or disease.
    Viruses,
    /// An icon of a cassette tape, representing voicemail messages.
    Voicemail,
    /// A volcano, representing eruption or natural phenomenon.
    Volcano,
    /// A volleyball, representing the sport.
    Volleyball,
    /// A speaker with high volume, representing loud audio.
    VolumeHigh,
    /// A speaker with low volume, representing soft audio.
    VolumeLow,
    /// A speaker without sound waves, indicating no volume.
    VolumeOff,
    /// A speaker with an `X`, indicating mute or no sound.
    VolumeXmark,
    /// The logo of Google Cardboard, a VR platform.
    VrCardboard,
    /// A capital letter 'W', representing the letter or something starting with
    /// W.
    W,
    /// A walkie-talkie, representing communication devices.
    WalkieTalkie,
    /// A wallet, representing money or finances.
    Wallet,
    /// A magic wand, representing magical effects.
    WandMagic,
    /// A magic wand with sparkles, indicating magical effects or settings.
    WandMagicSparkles,
    /// A magic wand with sparkles, indicating magic or settings.
    WandSparkles,
    /// A warehouse building, representing storage or logistics.
    Warehouse,
    /// A water droplet, representing liquid or hydration.
    Water,
    /// A water ladder, representing swimming pools or rescue.
    WaterLadder,
    /// A square wave, representing a waveform.
    WaveSquare,
    /// The logo of Web Awesome, representing the web development tool.
    WebAwesome,
    /// A hanging weight, representing heavy lifting or measurement.
    WeightHanging,
    /// A weight scale, representing measurement of weight.
    WeightScale,
    /// A stalk of wheat, representing grain or agriculture.
    WheatAwn,
    /// A circle with wheat and an exclamation mark, representing gluten alert.
    WheatAwnCircleExclamation,
    /// A simple wheelchair, representing accessibility.
    Wheelchair,
    /// A wheelchair with motion lines, representing mobility.
    WheelchairMove,
    /// A whiskey glass, representing alcohol or beverages.
    WhiskeyGlass,
    /// A signal icon, representing wireless internet connectivity.
    Wifi,
    /// A wind symbol, representing breeze or weather.
    Wind,
    /// A window being maximized, representing expansion.
    WindowMaximize,
    /// A window being minimized, representing reduction.
    WindowMinimize,
    /// A window being restored, representing reopening or resizing.
    WindowRestore,
    /// The logo of Windows, a popular operating system by Microsoft.
    Windows,
    /// A wine bottle, representing alcohol or beverages.
    WineBottle,
    /// A wine glass, representing drinking or celebration.
    WineGlass,
    /// An empty wine glass, representing a drink.
    WineGlassEmpty,
    /// The logo of Wizards of the Coast, representing the game company.
    WizardsOfTheCoast,
    /// The symbol for the South Korean won, indicating currency.
    WonSign,
    /// The logo of `WordPress`, a popular content management system.
    Wordpress,
    /// A worm, representing the animal or an insult.
    Worm,
    /// A wrench, representing tools or repair.
    Wrench,
    /// A capital letter `X`, representing the letter or close.
    X,
    /// An X-ray, representing medical imaging.
    XRay,
    /// The logo of Xbox, representing the gaming console.
    Xbox,
    /// A simple `X` mark, indicating error or cancellation.
    Xmark,
    /// Lines forming an `X`, representing rejection or closure.
    XmarksLines,
    /// The letter `Y`, representing the alphabet.
    Y,
    /// The symbol for the Japanese yen, indicating currency.
    YenSign,
    /// A yin-yang symbol, representing balance or duality.
    YinYang,
    /// The logo of `YouTube`, a video-sharing platform.
    Youtube,
    /// A capital letter 'Z', representing the letter or sleep.
    Z,
}

impl Icon {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Returns the Font Awesome class for the icon.
    pub fn class(&self) -> &str {
        match self {
            Icon::Zero => "0",
            Icon::One => "1",
            Icon::Two => "2",
            Icon::Three => "3",
            Icon::Four => "4",
            Icon::Five => "5",
            Icon::Six => "6",
            Icon::Seven => "7",
            Icon::Eight => "8",
            Icon::Nine => "9",
            Icon::A => "a",
            Icon::AccessibleIcon => "accessible-icon",
            Icon::AddressBook => "address-book",
            Icon::AddressCard => "address-card",
            Icon::Algolia => "algolia",
            Icon::AlignCenter => "align-center",
            Icon::AlignJustify => "align-justify",
            Icon::AlignLeft => "align-left",
            Icon::AlignRight => "align-right",
            Icon::Alipay => "alipay",
            Icon::AmazonPay => "amazon-pay",
            Icon::Anchor => "anchor",
            Icon::AnchorCircleCheck => "anchor-circle-check",
            Icon::AnchorCircleExclamation => "anchor-circle-exclamation",
            Icon::AnchorCircleXmark => "anchor-circle-xmark",
            Icon::AnchorLock => "anchor-lock",
            Icon::Android => "android",
            Icon::AngleDown => "angle-down",
            Icon::AngleLeft => "angle-left",
            Icon::AngleRight => "angle-right",
            Icon::AngleUp => "angle-up",
            Icon::AnglesDown => "angles-down",
            Icon::AnglesLeft => "angles-left",
            Icon::AnglesRight => "angles-right",
            Icon::AnglesUp => "angles-up",
            Icon::Ankh => "ankh",
            Icon::Apple => "apple",
            Icon::ApplePay => "apple-pay",
            Icon::AppleWhole => "apple-whole",
            Icon::Archway => "archway",
            Icon::ArrowDown => "arrow-down",
            Icon::ArrowDown19 => "arrow-down-1-9",
            Icon::ArrowDown91 => "arrow-down-9-1",
            Icon::ArrowDownAZ => "arrow-down-a-z",
            Icon::ArrowDownLong => "arrow-down-long",
            Icon::ArrowDownShortWide => "arrow-down-short-wide",
            Icon::ArrowDownUpAcrossLine => "arrow-down-up-across-line",
            Icon::ArrowDownUpLock => "arrow-down-up-lock",
            Icon::ArrowDownWideShort => "arrow-down-wide-short",
            Icon::ArrowDownZA => "arrow-down-z-a",
            Icon::ArrowLeft => "arrow-left",
            Icon::ArrowLeftLong => "arrow-left-long",
            Icon::ArrowPointer => "arrow-pointer",
            Icon::ArrowRight => "arrow-right",
            Icon::ArrowRightArrowLeft => "arrow-right-arrow-left",
            Icon::ArrowRightFromBracket => "arrow-right-from-bracket",
            Icon::ArrowRightLong => "arrow-right-long",
            Icon::ArrowRightToBracket => "arrow-right-to-bracket",
            Icon::ArrowRightToCity => "arrow-right-to-city",
            Icon::ArrowRotateLeft => "arrow-rotate-left",
            Icon::ArrowRotateRight => "arrow-rotate-right",
            Icon::ArrowTrendDown => "arrow-trend-down",
            Icon::ArrowTrendUp => "arrow-trend-up",
            Icon::ArrowTurnDown => "arrow-turn-down",
            Icon::ArrowTurnUp => "arrow-turn-up",
            Icon::ArrowUp => "arrow-up",
            Icon::ArrowUp19 => "arrow-up-1-9",
            Icon::ArrowUp91 => "arrow-up-9-1",
            Icon::ArrowUpAZ => "arrow-up-a-z",
            Icon::ArrowUpFromBracket => "arrow-up-from-bracket",
            Icon::ArrowUpFromGroundWater => "arrow-up-from-ground-water",
            Icon::ArrowUpFromWaterPump => "arrow-up-from-water-pump",
            Icon::ArrowUpLong => "arrow-up-long",
            Icon::ArrowUpRightDots => "arrow-up-right-dots",
            Icon::ArrowUpRightFromSquare => "arrow-up-right-from-square",
            Icon::ArrowUpShortWide => "arrow-up-short-wide",
            Icon::ArrowUpWideShort => "arrow-up-wide-short",
            Icon::ArrowUpZA => "arrow-up-z-a",
            Icon::ArrowsDownToLine => "arrows-down-to-line",
            Icon::ArrowsDownToPeople => "arrows-down-to-people",
            Icon::ArrowsLeftRight => "arrows-left-right",
            Icon::ArrowsLeftRightToLine => "arrows-left-right-to-line",
            Icon::ArrowsRotate => "arrows-rotate",
            Icon::ArrowsSpin => "arrows-spin",
            Icon::ArrowsSplitUpAndLeft => "arrows-split-up-and-left",
            Icon::ArrowsToCircle => "arrows-to-circle",
            Icon::ArrowsToDot => "arrows-to-dot",
            Icon::ArrowsToEye => "arrows-to-eye",
            Icon::ArrowsTurnRight => "arrows-turn-right",
            Icon::ArrowsTurnToDots => "arrows-turn-to-dots",
            Icon::ArrowsUpDown => "arrows-up-down",
            Icon::ArrowsUpDownLeftRight => "arrows-up-down-left-right",
            Icon::ArrowsUpToLine => "arrows-up-to-line",
            Icon::Asterisk => "asterisk",
            Icon::At => "at",
            Icon::Atom => "atom",
            Icon::AudioDescription => "audio-description",
            Icon::AustralSign => "austral-sign",
            Icon::Award => "award",
            Icon::B => "b",
            Icon::Baby => "baby",
            Icon::BabyCarriage => "baby-carriage",
            Icon::Backward => "backward",
            Icon::BackwardFast => "backward-fast",
            Icon::BackwardStep => "backward-step",
            Icon::Bacon => "bacon",
            Icon::Bacteria => "bacteria",
            Icon::Bacterium => "bacterium",
            Icon::BagShopping => "bag-shopping",
            Icon::Bahai => "bahai",
            Icon::BahtSign => "baht-sign",
            Icon::Ban => "ban",
            Icon::BanSmoking => "ban-smoking",
            Icon::Bandage => "bandage",
            Icon::BangladeshiTakaSign => "bangladeshi-taka-sign",
            Icon::Barcode => "barcode",
            Icon::Bars => "bars",
            Icon::BarsProgress => "bars-progress",
            Icon::BarsStaggered => "bars-staggered",
            Icon::Baseball => "baseball",
            Icon::BaseballBatBall => "baseball-bat-ball",
            Icon::BasketShopping => "basket-shopping",
            Icon::Basketball => "basketball",
            Icon::Bath => "bath",
            Icon::BatteryEmpty => "battery-empty",
            Icon::BatteryFull => "battery-full",
            Icon::BatteryHalf => "battery-half",
            Icon::BatteryQuarter => "battery-quarter",
            Icon::BatteryThreeQuarters => "battery-three-quarters",
            Icon::Bed => "bed",
            Icon::BedPulse => "bed-pulse",
            Icon::BeerMugEmpty => "beer-mug-empty",
            Icon::Bell => "bell",
            Icon::BellConcierge => "bell-concierge",
            Icon::BellSlash => "bell-slash",
            Icon::BezierCurve => "bezier-curve",
            Icon::Bicycle => "bicycle",
            Icon::Binoculars => "binoculars",
            Icon::Biohazard => "biohazard",
            Icon::Bitcoin => "bitcoin",
            Icon::BitcoinSign => "bitcoin-sign",
            Icon::Blender => "blender",
            Icon::BlenderPhone => "blender-phone",
            Icon::Blog => "blog",
            Icon::Bluetooth => "bluetooth",
            Icon::BluetoothB => "bluetooth-b",
            Icon::Bold => "bold",
            Icon::Bolt => "bolt",
            Icon::BoltLightning => "bolt-lightning",
            Icon::Bomb => "bomb",
            Icon::Bone => "bone",
            Icon::Bong => "bong",
            Icon::Book => "book",
            Icon::BookAtlas => "book-atlas",
            Icon::BookBible => "book-bible",
            Icon::BookBookmark => "book-bookmark",
            Icon::BookJournalWhills => "book-journal-whills",
            Icon::BookMedical => "book-medical",
            Icon::BookOpen => "book-open",
            Icon::BookOpenReader => "book-open-reader",
            Icon::BookQuran => "book-quran",
            Icon::BookSkull => "book-skull",
            Icon::BookTanakh => "book-tanakh",
            Icon::Bookmark => "bookmark",
            Icon::BorderAll => "border-all",
            Icon::BorderNone => "border-none",
            Icon::BorderTopLeft => "border-top-left",
            Icon::BoreHole => "bore-hole",
            Icon::BottleDroplet => "bottle-droplet",
            Icon::BottleWater => "bottle-water",
            Icon::BowlFood => "bowl-food",
            Icon::BowlRice => "bowl-rice",
            Icon::BowlingBall => "bowling-ball",
            Icon::Box => "box",
            Icon::BoxArchive => "box-archive",
            Icon::BoxOpen => "box-open",
            Icon::BoxTissue => "box-tissue",
            Icon::BoxesPacking => "boxes-packing",
            Icon::BoxesStacked => "boxes-stacked",
            Icon::Braille => "braille",
            Icon::Brain => "brain",
            Icon::BrazilianRealSign => "brazilian-real-sign",
            Icon::BreadSlice => "bread-slice",
            Icon::Bridge => "bridge",
            Icon::BridgeCircleCheck => "bridge-circle-check",
            Icon::BridgeCircleExclamation => "bridge-circle-exclamation",
            Icon::BridgeCircleXmark => "bridge-circle-xmark",
            Icon::BridgeLock => "bridge-lock",
            Icon::BridgeWater => "bridge-water",
            Icon::Briefcase => "briefcase",
            Icon::BriefcaseMedical => "briefcase-medical",
            Icon::Broom => "broom",
            Icon::BroomBall => "broom-ball",
            Icon::Brush => "brush",
            Icon::Btc => "btc",
            Icon::Bucket => "bucket",
            Icon::Bug => "bug",
            Icon::BugSlash => "bug-slash",
            Icon::Bugs => "bugs",
            Icon::Building => "building",
            Icon::BuildingCircleArrowRight => "building-circle-arrow-right",
            Icon::BuildingCircleCheck => "building-circle-check",
            Icon::BuildingCircleExclamation => "building-circle-exclamation",
            Icon::BuildingCircleXmark => "building-circle-xmark",
            Icon::BuildingColumns => "building-columns",
            Icon::BuildingFlag => "building-flag",
            Icon::BuildingLock => "building-lock",
            Icon::BuildingNgo => "building-ngo",
            Icon::BuildingShield => "building-shield",
            Icon::BuildingUn => "building-un",
            Icon::BuildingUser => "building-user",
            Icon::BuildingWheat => "building-wheat",
            Icon::Bullhorn => "bullhorn",
            Icon::Bullseye => "bullseye",
            Icon::Burger => "burger",
            Icon::Burst => "burst",
            Icon::Bus => "bus",
            Icon::BusSimple => "bus-simple",
            Icon::BusinessTime => "business-time",
            Icon::C => "c",
            Icon::CableCar => "cable-car",
            Icon::CakeCandles => "cake-candles",
            Icon::Calculator => "calculator",
            Icon::Calendar => "calendar",
            Icon::CalendarCheck => "calendar-check",
            Icon::CalendarDay => "calendar-day",
            Icon::CalendarDays => "calendar-days",
            Icon::CalendarMinus => "calendar-minus",
            Icon::CalendarPlus => "calendar-plus",
            Icon::CalendarWeek => "calendar-week",
            Icon::CalendarXmark => "calendar-xmark",
            Icon::Camera => "camera",
            Icon::CameraRetro => "camera-retro",
            Icon::CameraRotate => "camera-rotate",
            Icon::Campground => "campground",
            Icon::CandyCane => "candy-cane",
            Icon::Cannabis => "cannabis",
            Icon::Capsules => "capsules",
            Icon::Car => "car",
            Icon::CarBattery => "car-battery",
            Icon::CarBurst => "car-burst",
            Icon::CarOn => "car-on",
            Icon::CarRear => "car-rear",
            Icon::CarSide => "car-side",
            Icon::CarTunnel => "car-tunnel",
            Icon::Caravan => "caravan",
            Icon::CaretDown => "caret-down",
            Icon::CaretLeft => "caret-left",
            Icon::CaretRight => "caret-right",
            Icon::CaretUp => "caret-up",
            Icon::Carrot => "carrot",
            Icon::CartArrowDown => "cart-arrow-down",
            Icon::CartFlatbed => "cart-flatbed",
            Icon::CartFlatbedSuitcase => "cart-flatbed-suitcase",
            Icon::CartPlus => "cart-plus",
            Icon::CartShopping => "cart-shopping",
            Icon::CashRegister => "cash-register",
            Icon::Cat => "cat",
            Icon::CcAmazonPay => "cc-amazon-pay",
            Icon::CcAmex => "cc-amex",
            Icon::CcApplePay => "cc-apple-pay",
            Icon::CcDinersClub => "cc-diners-club",
            Icon::CcDiscover => "cc-discover",
            Icon::CcJcb => "cc-jcb",
            Icon::CcMastercard => "cc-mastercard",
            Icon::CcPaypal => "cc-paypal",
            Icon::CcStripe => "cc-stripe",
            Icon::CcVisa => "cc-visa",
            Icon::CediSign => "cedi-sign",
            Icon::CentSign => "cent-sign",
            Icon::Certificate => "certificate",
            Icon::Chair => "chair",
            Icon::Chalkboard => "chalkboard",
            Icon::ChalkboardUser => "chalkboard-user",
            Icon::ChampagneGlasses => "champagne-glasses",
            Icon::ChargingStation => "charging-station",
            Icon::ChartArea => "chart-area",
            Icon::ChartBar => "chart-bar",
            Icon::ChartColumn => "chart-column",
            Icon::ChartGantt => "chart-gantt",
            Icon::ChartLine => "chart-line",
            Icon::ChartPie => "chart-pie",
            Icon::ChartSimple => "chart-simple",
            Icon::Check => "check",
            Icon::CheckDouble => "check-double",
            Icon::CheckToSlot => "check-to-slot",
            Icon::Cheese => "cheese",
            Icon::Chess => "chess",
            Icon::ChessBishop => "chess-bishop",
            Icon::ChessBoard => "chess-board",
            Icon::ChessKing => "chess-king",
            Icon::ChessKnight => "chess-knight",
            Icon::ChessPawn => "chess-pawn",
            Icon::ChessQueen => "chess-queen",
            Icon::ChessRook => "chess-rook",
            Icon::ChevronDown => "chevron-down",
            Icon::ChevronLeft => "chevron-left",
            Icon::ChevronRight => "chevron-right",
            Icon::ChevronUp => "chevron-up",
            Icon::Child => "child",
            Icon::ChildCombatant => "child-combatant",
            Icon::ChildDress => "child-dress",
            Icon::ChildReaching => "child-reaching",
            Icon::Children => "children",
            Icon::Church => "church",
            Icon::Circle => "circle",
            Icon::CircleArrowDown => "circle-arrow-down",
            Icon::CircleArrowLeft => "circle-arrow-left",
            Icon::CircleArrowRight => "circle-arrow-right",
            Icon::CircleArrowUp => "circle-arrow-up",
            Icon::CircleCheck => "circle-check",
            Icon::CircleChevronDown => "circle-chevron-down",
            Icon::CircleChevronLeft => "circle-chevron-left",
            Icon::CircleChevronRight => "circle-chevron-right",
            Icon::CircleChevronUp => "circle-chevron-up",
            Icon::CircleDollarToSlot => "circle-dollar-to-slot",
            Icon::CircleDot => "circle-dot",
            Icon::CircleDown => "circle-down",
            Icon::CircleExclamation => "circle-exclamation",
            Icon::CircleH => "circle-h",
            Icon::CircleHalfStroke => "circle-half-stroke",
            Icon::CircleInfo => "circle-info",
            Icon::CircleLeft => "circle-left",
            Icon::CircleMinus => "circle-minus",
            Icon::CircleNodes => "circle-nodes",
            Icon::CircleNotch => "circle-notch",
            Icon::CirclePause => "circle-pause",
            Icon::CirclePlay => "circle-play",
            Icon::CirclePlus => "circle-plus",
            Icon::CircleQuestion => "circle-question",
            Icon::CircleRadiation => "circle-radiation",
            Icon::CircleRight => "circle-right",
            Icon::CircleStop => "circle-stop",
            Icon::CircleUp => "circle-up",
            Icon::CircleUser => "circle-user",
            Icon::CircleXmark => "circle-xmark",
            Icon::City => "city",
            Icon::Clapperboard => "clapperboard",
            Icon::Clipboard => "clipboard",
            Icon::ClipboardCheck => "clipboard-check",
            Icon::ClipboardList => "clipboard-list",
            Icon::ClipboardQuestion => "clipboard-question",
            Icon::ClipboardUser => "clipboard-user",
            Icon::Clock => "clock",
            Icon::ClockRotateLeft => "clock-rotate-left",
            Icon::Clone => "clone",
            Icon::ClosedCaptioning => "closed-captioning",
            Icon::Cloud => "cloud",
            Icon::CloudArrowDown => "cloud-arrow-down",
            Icon::CloudArrowUp => "cloud-arrow-up",
            Icon::CloudBolt => "cloud-bolt",
            Icon::CloudMeatball => "cloud-meatball",
            Icon::CloudMoon => "cloud-moon",
            Icon::CloudMoonRain => "cloud-moon-rain",
            Icon::CloudRain => "cloud-rain",
            Icon::CloudShowersHeavy => "cloud-showers-heavy",
            Icon::CloudShowersWater => "cloud-showers-water",
            Icon::CloudSun => "cloud-sun",
            Icon::CloudSunRain => "cloud-sun-rain",
            Icon::Cloudflare => "cloudflare",
            Icon::Clover => "clover",
            Icon::Code => "code",
            Icon::CodeBranch => "code-branch",
            Icon::CodeCommit => "code-commit",
            Icon::CodeCompare => "code-compare",
            Icon::CodeFork => "code-fork",
            Icon::CodeMerge => "code-merge",
            Icon::CodePullRequest => "code-pull-request",
            Icon::Codepen => "codepen",
            Icon::Coins => "coins",
            Icon::ColonSign => "colon-sign",
            Icon::Comment => "comment",
            Icon::CommentDollar => "comment-dollar",
            Icon::CommentDots => "comment-dots",
            Icon::CommentMedical => "comment-medical",
            Icon::CommentSlash => "comment-slash",
            Icon::CommentSms => "comment-sms",
            Icon::Comments => "comments",
            Icon::CommentsDollar => "comments-dollar",
            Icon::CompactDisc => "compact-disc",
            Icon::Compass => "compass",
            Icon::CompassDrafting => "compass-drafting",
            Icon::Compress => "compress",
            Icon::Computer => "computer",
            Icon::ComputerMouse => "computer-mouse",
            Icon::Cookie => "cookie",
            Icon::CookieBite => "cookie-bite",
            Icon::Copy => "copy",
            Icon::Copyright => "copyright",
            Icon::Couch => "couch",
            Icon::Cow => "cow",
            Icon::CreditCard => "credit-card",
            Icon::CriticalRole => "critical-role",
            Icon::Crop => "crop",
            Icon::CropSimple => "crop-simple",
            Icon::Cross => "cross",
            Icon::Crosshairs => "crosshairs",
            Icon::Crow => "crow",
            Icon::Crown => "crown",
            Icon::Crutch => "crutch",
            Icon::CruzeiroSign => "cruzeiro-sign",
            Icon::Cube => "cube",
            Icon::Cubes => "cubes",
            Icon::CubesStacked => "cubes-stacked",
            Icon::D => "d",
            Icon::DAndD => "d-and-d",
            Icon::DAndDBeyond => "d-and-d-beyond",
            Icon::Database => "database",
            Icon::DeleteLeft => "delete-left",
            Icon::Democrat => "democrat",
            Icon::Desktop => "desktop",
            Icon::Dharmachakra => "dharmachakra",
            Icon::DiagramNext => "diagram-next",
            Icon::DiagramPredecessor => "diagram-predecessor",
            Icon::DiagramProject => "diagram-project",
            Icon::DiagramSuccessor => "diagram-successor",
            Icon::Diamond => "diamond",
            Icon::DiamondTurnRight => "diamond-turn-right",
            Icon::Dice => "dice",
            Icon::DiceD20 => "dice-d20",
            Icon::DiceD6 => "dice-d6",
            Icon::DiceFive => "dice-five",
            Icon::DiceFour => "dice-four",
            Icon::DiceOne => "dice-one",
            Icon::DiceSix => "dice-six",
            Icon::DiceThree => "dice-three",
            Icon::DiceTwo => "dice-two",
            Icon::Discord => "discord",
            Icon::Disease => "disease",
            Icon::Display => "display",
            Icon::Divide => "divide",
            Icon::Dna => "dna",
            Icon::Docker => "docker",
            Icon::Dog => "dog",
            Icon::DollarSign => "dollar-sign",
            Icon::Dolly => "dolly",
            Icon::DongSign => "dong-sign",
            Icon::DoorClosed => "door-closed",
            Icon::DoorOpen => "door-open",
            Icon::Dove => "dove",
            Icon::DownLeftAndUpRightToCenter => "down-left-and-up-right-to-center",
            Icon::DownLong => "down-long",
            Icon::Download => "download",
            Icon::Dragon => "dragon",
            Icon::DrawPolygon => "draw-polygon",
            Icon::Dribbble => "dribbble",
            Icon::Dropbox => "dropbox",
            Icon::Droplet => "droplet",
            Icon::DropletSlash => "droplet-slash",
            Icon::Drum => "drum",
            Icon::DrumSteelpan => "drum-steelpan",
            Icon::DrumstickBite => "drumstick-bite",
            Icon::Dumbbell => "dumbbell",
            Icon::Dumpster => "dumpster",
            Icon::DumpsterFire => "dumpster-fire",
            Icon::Dungeon => "dungeon",
            Icon::E => "e",
            Icon::EarDeaf => "ear-deaf",
            Icon::EarListen => "ear-listen",
            Icon::EarthAfrica => "earth-africa",
            Icon::EarthAmericas => "earth-americas",
            Icon::EarthAsia => "earth-asia",
            Icon::EarthEurope => "earth-europe",
            Icon::EarthOceania => "earth-oceania",
            Icon::Egg => "egg",
            Icon::Eject => "eject",
            Icon::Elevator => "elevator",
            Icon::Ellipsis => "ellipsis",
            Icon::EllipsisVertical => "ellipsis-vertical",
            Icon::Envelope => "envelope",
            Icon::EnvelopeCircleCheck => "envelope-circle-check",
            Icon::EnvelopeOpen => "envelope-open",
            Icon::EnvelopeOpenText => "envelope-open-text",
            Icon::EnvelopesBulk => "envelopes-bulk",
            Icon::Equals => "equals",
            Icon::Eraser => "eraser",
            Icon::Ethereum => "ethereum",
            Icon::Ethernet => "ethernet",
            Icon::EuroSign => "euro-sign",
            Icon::Exclamation => "exclamation",
            Icon::Expand => "expand",
            Icon::Explosion => "explosion",
            Icon::Eye => "eye",
            Icon::EyeDropper => "eye-dropper",
            Icon::EyeLowVision => "eye-low-vision",
            Icon::EyeSlash => "eye-slash",
            Icon::F => "f",
            Icon::FaceAngry => "face-angry",
            Icon::FaceDizzy => "face-dizzy",
            Icon::FaceFlushed => "face-flushed",
            Icon::FaceFrown => "face-frown",
            Icon::FaceFrownOpen => "face-frown-open",
            Icon::FaceGrimace => "face-grimace",
            Icon::FaceGrin => "face-grin",
            Icon::FaceGrinBeam => "face-grin-beam",
            Icon::FaceGrinBeamSweat => "face-grin-beam-sweat",
            Icon::FaceGrinHearts => "face-grin-hearts",
            Icon::FaceGrinSquint => "face-grin-squint",
            Icon::FaceGrinSquintTears => "face-grin-squint-tears",
            Icon::FaceGrinStars => "face-grin-stars",
            Icon::FaceGrinTears => "face-grin-tears",
            Icon::FaceGrinTongue => "face-grin-tongue",
            Icon::FaceGrinTongueSquint => "face-grin-tongue-squint",
            Icon::FaceGrinTongueWink => "face-grin-tongue-wink",
            Icon::FaceGrinWide => "face-grin-wide",
            Icon::FaceGrinWink => "face-grin-wink",
            Icon::FaceKiss => "face-kiss",
            Icon::FaceKissBeam => "face-kiss-beam",
            Icon::FaceKissWinkHeart => "face-kiss-wink-heart",
            Icon::FaceLaugh => "face-laugh",
            Icon::FaceLaughBeam => "face-laugh-beam",
            Icon::FaceLaughSquint => "face-laugh-squint",
            Icon::FaceLaughWink => "face-laugh-wink",
            Icon::FaceMeh => "face-meh",
            Icon::FaceMehBlank => "face-meh-blank",
            Icon::FaceRollingEyes => "face-rolling-eyes",
            Icon::FaceSadCry => "face-sad-cry",
            Icon::FaceSadTear => "face-sad-tear",
            Icon::FaceSmile => "face-smile",
            Icon::FaceSmileBeam => "face-smile-beam",
            Icon::FaceSmileWink => "face-smile-wink",
            Icon::FaceSurprise => "face-surprise",
            Icon::FaceTired => "face-tired",
            Icon::Facebook => "facebook",
            Icon::Fan => "fan",
            Icon::FantasyFlightGames => "fantasy-flight-games",
            Icon::Faucet => "faucet",
            Icon::FaucetDrip => "faucet-drip",
            Icon::Fax => "fax",
            Icon::Feather => "feather",
            Icon::FeatherPointed => "feather-pointed",
            Icon::Ferry => "ferry",
            Icon::Figma => "figma",
            Icon::File => "file",
            Icon::FileArrowDown => "file-arrow-down",
            Icon::FileArrowUp => "file-arrow-up",
            Icon::FileAudio => "file-audio",
            Icon::FileCircleCheck => "file-circle-check",
            Icon::FileCircleExclamation => "file-circle-exclamation",
            Icon::FileCircleMinus => "file-circle-minus",
            Icon::FileCirclePlus => "file-circle-plus",
            Icon::FileCircleQuestion => "file-circle-question",
            Icon::FileCircleXmark => "file-circle-xmark",
            Icon::FileCode => "file-code",
            Icon::FileContract => "file-contract",
            Icon::FileCsv => "file-csv",
            Icon::FileExcel => "file-excel",
            Icon::FileExport => "file-export",
            Icon::FileImage => "file-image",
            Icon::FileImport => "file-import",
            Icon::FileInvoice => "file-invoice",
            Icon::FileInvoiceDollar => "file-invoice-dollar",
            Icon::FileLines => "file-lines",
            Icon::FileMedical => "file-medical",
            Icon::FilePdf => "file-pdf",
            Icon::FilePen => "file-pen",
            Icon::FilePowerpoint => "file-powerpoint",
            Icon::FilePrescription => "file-prescription",
            Icon::FileShield => "file-shield",
            Icon::FileSignature => "file-signature",
            Icon::FileVideo => "file-video",
            Icon::FileWaveform => "file-waveform",
            Icon::FileWord => "file-word",
            Icon::FileZipper => "file-zipper",
            Icon::Fill => "fill",
            Icon::FillDrip => "fill-drip",
            Icon::Film => "film",
            Icon::Filter => "filter",
            Icon::FilterCircleDollar => "filter-circle-dollar",
            Icon::FilterCircleXmark => "filter-circle-xmark",
            Icon::Fingerprint => "fingerprint",
            Icon::Fire => "fire",
            Icon::FireBurner => "fire-burner",
            Icon::FireExtinguisher => "fire-extinguisher",
            Icon::FireFlameCurved => "fire-flame-curved",
            Icon::FireFlameSimple => "fire-flame-simple",
            Icon::Fish => "fish",
            Icon::FishFins => "fish-fins",
            Icon::Flag => "flag",
            Icon::FlagCheckered => "flag-checkered",
            Icon::FlagUsa => "flag-usa",
            Icon::Flask => "flask",
            Icon::FlaskVial => "flask-vial",
            Icon::FloppyDisk => "floppy-disk",
            Icon::FlorinSign => "florin-sign",
            Icon::Folder => "folder",
            Icon::FolderClosed => "folder-closed",
            Icon::FolderMinus => "folder-minus",
            Icon::FolderOpen => "folder-open",
            Icon::FolderPlus => "folder-plus",
            Icon::FolderTree => "folder-tree",
            Icon::Font => "font",
            Icon::FontAwesome => "font-awesome",
            Icon::Football => "football",
            Icon::Forward => "forward",
            Icon::ForwardFast => "forward-fast",
            Icon::ForwardStep => "forward-step",
            Icon::FrancSign => "franc-sign",
            Icon::Frog => "frog",
            Icon::Futbol => "futbol",
            Icon::G => "g",
            Icon::GalacticRepublic => "galactic-republic",
            Icon::GalacticSenate => "galactic-senate",
            Icon::Gamepad => "gamepad",
            Icon::GasPump => "gas-pump",
            Icon::Gauge => "gauge",
            Icon::GaugeHigh => "gauge-high",
            Icon::GaugeSimple => "gauge-simple",
            Icon::GaugeSimpleHigh => "gauge-simple-high",
            Icon::Gavel => "gavel",
            Icon::Gear => "gear",
            Icon::Gears => "gears",
            Icon::Gem => "gem",
            Icon::Genderless => "genderless",
            Icon::Gg => "gg",
            Icon::GgCircle => "gg-circle",
            Icon::Ghost => "ghost",
            Icon::Gift => "gift",
            Icon::Gifts => "gifts",
            Icon::Github => "github",
            Icon::GlassWater => "glass-water",
            Icon::GlassWaterDroplet => "glass-water-droplet",
            Icon::Glasses => "glasses",
            Icon::Globe => "globe",
            Icon::GolfBallTee => "golf-ball-tee",
            Icon::Google => "google",
            Icon::GooglePay => "google-pay",
            Icon::GoogleWallet => "google-wallet",
            Icon::Gopuram => "gopuram",
            Icon::GraduationCap => "graduation-cap",
            Icon::GreaterThan => "greater-than",
            Icon::GreaterThanEqual => "greater-than-equal",
            Icon::Grip => "grip",
            Icon::GripLines => "grip-lines",
            Icon::GripLinesVertical => "grip-lines-vertical",
            Icon::GripVertical => "grip-vertical",
            Icon::GroupArrowsRotate => "group-arrows-rotate",
            Icon::GuaraniSign => "guarani-sign",
            Icon::Guitar => "guitar",
            Icon::Gun => "gun",
            Icon::H => "h",
            Icon::Hammer => "hammer",
            Icon::Hamsa => "hamsa",
            Icon::Hand => "hand",
            Icon::HandBackFist => "hand-back-fist",
            Icon::HandDots => "hand-dots",
            Icon::HandFist => "hand-fist",
            Icon::HandHolding => "hand-holding",
            Icon::HandHoldingDollar => "hand-holding-dollar",
            Icon::HandHoldingDroplet => "hand-holding-droplet",
            Icon::HandHoldingHand => "hand-holding-hand",
            Icon::HandHoldingHeart => "hand-holding-heart",
            Icon::HandHoldingMedical => "hand-holding-medical",
            Icon::HandLizard => "hand-lizard",
            Icon::HandMiddleFinger => "hand-middle-finger",
            Icon::HandPeace => "hand-peace",
            Icon::HandPointDown => "hand-point-down",
            Icon::HandPointLeft => "hand-point-left",
            Icon::HandPointRight => "hand-point-right",
            Icon::HandPointUp => "hand-point-up",
            Icon::HandPointer => "hand-pointer",
            Icon::HandScissors => "hand-scissors",
            Icon::HandSparkles => "hand-sparkles",
            Icon::HandSpock => "hand-spock",
            Icon::Handcuffs => "handcuffs",
            Icon::Hands => "hands",
            Icon::HandsAslInterpreting => "hands-asl-interpreting",
            Icon::HandsBound => "hands-bound",
            Icon::HandsBubbles => "hands-bubbles",
            Icon::HandsClapping => "hands-clapping",
            Icon::HandsHolding => "hands-holding",
            Icon::HandsHoldingChild => "hands-holding-child",
            Icon::HandsHoldingCircle => "hands-holding-circle",
            Icon::HandsPraying => "hands-praying",
            Icon::Handshake => "handshake",
            Icon::HandshakeAngle => "handshake-angle",
            Icon::HandshakeSimple => "handshake-simple",
            Icon::HandshakeSimpleSlash => "handshake-simple-slash",
            Icon::HandshakeSlash => "handshake-slash",
            Icon::Hanukiah => "hanukiah",
            Icon::HardDrive => "hard-drive",
            Icon::Hashtag => "hashtag",
            Icon::HatCowboy => "hat-cowboy",
            Icon::HatCowboySide => "hat-cowboy-side",
            Icon::HatWizard => "hat-wizard",
            Icon::HeadSideCough => "head-side-cough",
            Icon::HeadSideCoughSlash => "head-side-cough-slash",
            Icon::HeadSideMask => "head-side-mask",
            Icon::HeadSideVirus => "head-side-virus",
            Icon::Heading => "heading",
            Icon::Headphones => "headphones",
            Icon::HeadphonesSimple => "headphones-simple",
            Icon::Headset => "headset",
            Icon::Heart => "heart",
            Icon::HeartCircleBolt => "heart-circle-bolt",
            Icon::HeartCircleCheck => "heart-circle-check",
            Icon::HeartCircleExclamation => "heart-circle-exclamation",
            Icon::HeartCircleMinus => "heart-circle-minus",
            Icon::HeartCirclePlus => "heart-circle-plus",
            Icon::HeartCircleXmark => "heart-circle-xmark",
            Icon::HeartCrack => "heart-crack",
            Icon::HeartPulse => "heart-pulse",
            Icon::Helicopter => "helicopter",
            Icon::HelicopterSymbol => "helicopter-symbol",
            Icon::HelmetSafety => "helmet-safety",
            Icon::HelmetUn => "helmet-un",
            Icon::Highlighter => "highlighter",
            Icon::HillAvalanche => "hill-avalanche",
            Icon::HillRockslide => "hill-rockslide",
            Icon::Hippo => "hippo",
            Icon::HockeyPuck => "hockey-puck",
            Icon::HollyBerry => "holly-berry",
            Icon::Horse => "horse",
            Icon::HorseHead => "horse-head",
            Icon::Hospital => "hospital",
            Icon::HospitalUser => "hospital-user",
            Icon::HotTubPerson => "hot-tub-person",
            Icon::Hotdog => "hotdog",
            Icon::Hotel => "hotel",
            Icon::Hourglass => "hourglass",
            Icon::HourglassEnd => "hourglass-end",
            Icon::HourglassHalf => "hourglass-half",
            Icon::HourglassStart => "hourglass-start",
            Icon::House => "house",
            Icon::HouseChimney => "house-chimney",
            Icon::HouseChimneyCrack => "house-chimney-crack",
            Icon::HouseChimneyMedical => "house-chimney-medical",
            Icon::HouseChimneyUser => "house-chimney-user",
            Icon::HouseChimneyWindow => "house-chimney-window",
            Icon::HouseCircleCheck => "house-circle-check",
            Icon::HouseCircleExclamation => "house-circle-exclamation",
            Icon::HouseCircleXmark => "house-circle-xmark",
            Icon::HouseCrack => "house-crack",
            Icon::HouseFire => "house-fire",
            Icon::HouseFlag => "house-flag",
            Icon::HouseFloodWater => "house-flood-water",
            Icon::HouseFloodWaterCircleArrowRight => "house-flood-water-circle-arrow-right",
            Icon::HouseLaptop => "house-laptop",
            Icon::HouseLock => "house-lock",
            Icon::HouseMedical => "house-medical",
            Icon::HouseMedicalCircleCheck => "house-medical-circle-check",
            Icon::HouseMedicalCircleExclamation => "house-medical-circle-exclamation",
            Icon::HouseMedicalCircleXmark => "house-medical-circle-xmark",
            Icon::HouseMedicalFlag => "house-medical-flag",
            Icon::HouseSignal => "house-signal",
            Icon::HouseTsunami => "house-tsunami",
            Icon::HouseUser => "house-user",
            Icon::HryvniaSign => "hryvnia-sign",
            Icon::Hurricane => "hurricane",
            Icon::I => "i",
            Icon::ICursor => "i-cursor",
            Icon::IceCream => "ice-cream",
            Icon::Icicles => "icicles",
            Icon::Icons => "icons",
            Icon::IdBadge => "id-badge",
            Icon::IdCard => "id-card",
            Icon::IdCardClip => "id-card-clip",
            Icon::Igloo => "igloo",
            Icon::Image => "image",
            Icon::ImagePortrait => "image-portrait",
            Icon::Images => "images",
            Icon::Inbox => "inbox",
            Icon::Indent => "indent",
            Icon::IndianRupeeSign => "indian-rupee-sign",
            Icon::Industry => "industry",
            Icon::Infinity => "infinity",
            Icon::Info => "info",
            Icon::Instagram => "instagram",
            Icon::Italic => "italic",
            Icon::J => "j",
            Icon::Jar => "jar",
            Icon::JarWheat => "jar-wheat",
            Icon::Jedi => "jedi",
            Icon::JediOrder => "jedi-order",
            Icon::JetFighter => "jet-fighter",
            Icon::JetFighterUp => "jet-fighter-up",
            Icon::Joint => "joint",
            Icon::JugDetergent => "jug-detergent",
            Icon::K => "k",
            Icon::Kaaba => "kaaba",
            Icon::Key => "key",
            Icon::Keyboard => "keyboard",
            Icon::Khanda => "khanda",
            Icon::Kickstarter => "kickstarter",
            Icon::KipSign => "kip-sign",
            Icon::KitMedical => "kit-medical",
            Icon::KitchenSet => "kitchen-set",
            Icon::KiwiBird => "kiwi-bird",
            Icon::L => "l",
            Icon::LandMineOn => "land-mine-on",
            Icon::Landmark => "landmark",
            Icon::LandmarkDome => "landmark-dome",
            Icon::LandmarkFlag => "landmark-flag",
            Icon::Language => "language",
            Icon::Laptop => "laptop",
            Icon::LaptopCode => "laptop-code",
            Icon::LaptopFile => "laptop-file",
            Icon::LaptopMedical => "laptop-medical",
            Icon::LariSign => "lari-sign",
            Icon::LayerGroup => "layer-group",
            Icon::Leaf => "leaf",
            Icon::LeftLong => "left-long",
            Icon::LeftRight => "left-right",
            Icon::Lemon => "lemon",
            Icon::LessThan => "less-than",
            Icon::LessThanEqual => "less-than-equal",
            Icon::LifeRing => "life-ring",
            Icon::Lightbulb => "lightbulb",
            Icon::LinesLeaning => "lines-leaning",
            Icon::Link => "link",
            Icon::LinkSlash => "link-slash",
            Icon::Linkedin => "linkedin",
            Icon::LiraSign => "lira-sign",
            Icon::List => "list",
            Icon::ListCheck => "list-check",
            Icon::ListOl => "list-ol",
            Icon::ListUl => "list-ul",
            Icon::LitecoinSign => "litecoin-sign",
            Icon::LocationArrow => "location-arrow",
            Icon::LocationCrosshairs => "location-crosshairs",
            Icon::LocationDot => "location-dot",
            Icon::LocationPin => "location-pin",
            Icon::LocationPinLock => "location-pin-lock",
            Icon::Lock => "lock",
            Icon::LockOpen => "lock-open",
            Icon::Locust => "locust",
            Icon::Lungs => "lungs",
            Icon::LungsVirus => "lungs-virus",
            Icon::M => "m",
            Icon::Magnet => "magnet",
            Icon::MagnifyingGlass => "magnifying-glass",
            Icon::MagnifyingGlassArrowRight => "magnifying-glass-arrow-right",
            Icon::MagnifyingGlassChart => "magnifying-glass-chart",
            Icon::MagnifyingGlassDollar => "magnifying-glass-dollar",
            Icon::MagnifyingGlassLocation => "magnifying-glass-location",
            Icon::MagnifyingGlassMinus => "magnifying-glass-minus",
            Icon::MagnifyingGlassPlus => "magnifying-glass-plus",
            Icon::ManatSign => "manat-sign",
            Icon::Map => "map",
            Icon::MapLocation => "map-location",
            Icon::MapLocationDot => "map-location-dot",
            Icon::MapPin => "map-pin",
            Icon::Marker => "marker",
            Icon::Mars => "mars",
            Icon::MarsAndVenus => "mars-and-venus",
            Icon::MarsAndVenusBurst => "mars-and-venus-burst",
            Icon::MarsDouble => "mars-double",
            Icon::MarsStroke => "mars-stroke",
            Icon::MarsStrokeRight => "mars-stroke-right",
            Icon::MarsStrokeUp => "mars-stroke-up",
            Icon::MartiniGlass => "martini-glass",
            Icon::MartiniGlassCitrus => "martini-glass-citrus",
            Icon::MartiniGlassEmpty => "martini-glass-empty",
            Icon::Mask => "mask",
            Icon::MaskFace => "mask-face",
            Icon::MaskVentilator => "mask-ventilator",
            Icon::MasksTheater => "masks-theater",
            Icon::MattressPillow => "mattress-pillow",
            Icon::Maximize => "maximize",
            Icon::Medal => "medal",
            Icon::Medium => "medium",
            Icon::Memory => "memory",
            Icon::Menorah => "menorah",
            Icon::Mercury => "mercury",
            Icon::Message => "message",
            Icon::Meteor => "meteor",
            Icon::Microchip => "microchip",
            Icon::Microphone => "microphone",
            Icon::MicrophoneLines => "microphone-lines",
            Icon::MicrophoneLinesSlash => "microphone-lines-slash",
            Icon::MicrophoneSlash => "microphone-slash",
            Icon::Microscope => "microscope",
            Icon::MillSign => "mill-sign",
            Icon::Minimize => "minimize",
            Icon::Minus => "minus",
            Icon::Mitten => "mitten",
            Icon::Mobile => "mobile",
            Icon::MobileButton => "mobile-button",
            Icon::MobileRetro => "mobile-retro",
            Icon::MobileScreen => "mobile-screen",
            Icon::MobileScreenButton => "mobile-screen-button",
            Icon::MoneyBill => "money-bill",
            Icon::MoneyBill1 => "money-bill-1",
            Icon::MoneyBill1Wave => "money-bill-1-wave",
            Icon::MoneyBillTransfer => "money-bill-transfer",
            Icon::MoneyBillTrendUp => "money-bill-trend-up",
            Icon::MoneyBillWave => "money-bill-wave",
            Icon::MoneyBillWheat => "money-bill-wheat",
            Icon::MoneyBills => "money-bills",
            Icon::MoneyCheck => "money-check",
            Icon::MoneyCheckDollar => "money-check-dollar",
            Icon::Monument => "monument",
            Icon::Moon => "moon",
            Icon::MortarPestle => "mortar-pestle",
            Icon::Mosque => "mosque",
            Icon::Mosquito => "mosquito",
            Icon::MosquitoNet => "mosquito-net",
            Icon::Motorcycle => "motorcycle",
            Icon::Mound => "mound",
            Icon::Mountain => "mountain",
            Icon::MountainCity => "mountain-city",
            Icon::MountainSun => "mountain-sun",
            Icon::MugHot => "mug-hot",
            Icon::MugSaucer => "mug-saucer",
            Icon::Music => "music",
            Icon::N => "n",
            Icon::NairaSign => "naira-sign",
            Icon::Napster => "napster",
            Icon::NetworkWired => "network-wired",
            Icon::Neuter => "neuter",
            Icon::Newspaper => "newspaper",
            Icon::NfcDirectional => "nfc-directional",
            Icon::NfcSymbol => "nfc-symbol",
            Icon::NotEqual => "not-equal",
            Icon::Notdef => "notdef",
            Icon::NoteSticky => "note-sticky",
            Icon::NotesMedical => "notes-medical",
            Icon::O => "o",
            Icon::ObjectGroup => "object-group",
            Icon::ObjectUngroup => "object-ungroup",
            Icon::OilCan => "oil-can",
            Icon::OilWell => "oil-well",
            Icon::OldRepublic => "old-republic",
            Icon::Om => "om",
            Icon::Otter => "otter",
            Icon::Outdent => "outdent",
            Icon::P => "p",
            Icon::Pager => "pager",
            Icon::PaintRoller => "paint-roller",
            Icon::Paintbrush => "paintbrush",
            Icon::Palette => "palette",
            Icon::Pallet => "pallet",
            Icon::Panorama => "panorama",
            Icon::PaperPlane => "paper-plane",
            Icon::Paperclip => "paperclip",
            Icon::ParachuteBox => "parachute-box",
            Icon::Paragraph => "paragraph",
            Icon::Passport => "passport",
            Icon::Paste => "paste",
            Icon::Pause => "pause",
            Icon::Paw => "paw",
            Icon::Paypal => "paypal",
            Icon::Peace => "peace",
            Icon::Pen => "pen",
            Icon::PenClip => "pen-clip",
            Icon::PenFancy => "pen-fancy",
            Icon::PenNib => "pen-nib",
            Icon::PenRuler => "pen-ruler",
            Icon::PenToSquare => "pen-to-square",
            Icon::Pencil => "pencil",
            Icon::PeopleArrows => "people-arrows",
            Icon::PeopleCarryBox => "people-carry-box",
            Icon::PeopleGroup => "people-group",
            Icon::PeopleLine => "people-line",
            Icon::PeoplePulling => "people-pulling",
            Icon::PeopleRobbery => "people-robbery",
            Icon::PeopleRoof => "people-roof",
            Icon::PepperHot => "pepper-hot",
            Icon::Percent => "percent",
            Icon::Person => "person",
            Icon::PersonArrowDownToLine => "person-arrow-down-to-line",
            Icon::PersonArrowUpFromLine => "person-arrow-up-from-line",
            Icon::PersonBiking => "person-biking",
            Icon::PersonBooth => "person-booth",
            Icon::PersonBreastfeeding => "person-breastfeeding",
            Icon::PersonBurst => "person-burst",
            Icon::PersonCane => "person-cane",
            Icon::PersonChalkboard => "person-chalkboard",
            Icon::PersonCircleCheck => "person-circle-check",
            Icon::PersonCircleExclamation => "person-circle-exclamation",
            Icon::PersonCircleMinus => "person-circle-minus",
            Icon::PersonCirclePlus => "person-circle-plus",
            Icon::PersonCircleQuestion => "person-circle-question",
            Icon::PersonCircleXmark => "person-circle-xmark",
            Icon::PersonDigging => "person-digging",
            Icon::PersonDotsFromLine => "person-dots-from-line",
            Icon::PersonDress => "person-dress",
            Icon::PersonDressBurst => "person-dress-burst",
            Icon::PersonDrowning => "person-drowning",
            Icon::PersonFalling => "person-falling",
            Icon::PersonFallingBurst => "person-falling-burst",
            Icon::PersonHalfDress => "person-half-dress",
            Icon::PersonHarassing => "person-harassing",
            Icon::PersonHiking => "person-hiking",
            Icon::PersonMilitaryPointing => "person-military-pointing",
            Icon::PersonMilitaryRifle => "person-military-rifle",
            Icon::PersonMilitaryToPerson => "person-military-to-person",
            Icon::PersonPraying => "person-praying",
            Icon::PersonPregnant => "person-pregnant",
            Icon::PersonRays => "person-rays",
            Icon::PersonRifle => "person-rifle",
            Icon::PersonRunning => "person-running",
            Icon::PersonShelter => "person-shelter",
            Icon::PersonSkating => "person-skating",
            Icon::PersonSkiing => "person-skiing",
            Icon::PersonSkiingNordic => "person-skiing-nordic",
            Icon::PersonSnowboarding => "person-snowboarding",
            Icon::PersonSwimming => "person-swimming",
            Icon::PersonThroughWindow => "person-through-window",
            Icon::PersonWalking => "person-walking",
            Icon::PersonWalkingArrowLoopLeft => "person-walking-arrow-loop-left",
            Icon::PersonWalkingArrowRight => "person-walking-arrow-right",
            Icon::PersonWalkingDashedLineArrowRight => "person-walking-dashed-line-arrow-right",
            Icon::PersonWalkingLuggage => "person-walking-luggage",
            Icon::PersonWalkingWithCane => "person-walking-with-cane",
            Icon::PesetaSign => "peseta-sign",
            Icon::PesoSign => "peso-sign",
            Icon::Phone => "phone",
            Icon::PhoneFlip => "phone-flip",
            Icon::PhoneSlash => "phone-slash",
            Icon::PhoneVolume => "phone-volume",
            Icon::PhotoFilm => "photo-film",
            Icon::PiggyBank => "piggy-bank",
            Icon::Pills => "pills",
            Icon::PizzaSlice => "pizza-slice",
            Icon::PlaceOfWorship => "place-of-worship",
            Icon::Plane => "plane",
            Icon::PlaneArrival => "plane-arrival",
            Icon::PlaneCircleCheck => "plane-circle-check",
            Icon::PlaneCircleExclamation => "plane-circle-exclamation",
            Icon::PlaneCircleXmark => "plane-circle-xmark",
            Icon::PlaneDeparture => "plane-departure",
            Icon::PlaneLock => "plane-lock",
            Icon::PlaneSlash => "plane-slash",
            Icon::PlaneUp => "plane-up",
            Icon::PlantWilt => "plant-wilt",
            Icon::PlateWheat => "plate-wheat",
            Icon::Play => "play",
            Icon::Playstation => "playstation",
            Icon::Plug => "plug",
            Icon::PlugCircleBolt => "plug-circle-bolt",
            Icon::PlugCircleCheck => "plug-circle-check",
            Icon::PlugCircleExclamation => "plug-circle-exclamation",
            Icon::PlugCircleMinus => "plug-circle-minus",
            Icon::PlugCirclePlus => "plug-circle-plus",
            Icon::PlugCircleXmark => "plug-circle-xmark",
            Icon::Plus => "plus",
            Icon::PlusMinus => "plus-minus",
            Icon::Podcast => "podcast",
            Icon::Poo => "poo",
            Icon::PooStorm => "poo-storm",
            Icon::Poop => "poop",
            Icon::PowerOff => "power-off",
            Icon::Prescription => "prescription",
            Icon::PrescriptionBottle => "prescription-bottle",
            Icon::PrescriptionBottleMedical => "prescription-bottle-medical",
            Icon::Print => "print",
            Icon::PumpMedical => "pump-medical",
            Icon::PumpSoap => "pump-soap",
            Icon::PuzzlePiece => "puzzle-piece",
            Icon::Q => "q",
            Icon::Qrcode => "qrcode",
            Icon::Question => "question",
            Icon::QuoteLeft => "quote-left",
            Icon::QuoteRight => "quote-right",
            Icon::R => "r",
            Icon::Radiation => "radiation",
            Icon::Radio => "radio",
            Icon::Rainbow => "rainbow",
            Icon::RankingStar => "ranking-star",
            Icon::Receipt => "receipt",
            Icon::RecordVinyl => "record-vinyl",
            Icon::RectangleAd => "rectangle-ad",
            Icon::RectangleList => "rectangle-list",
            Icon::RectangleXmark => "rectangle-xmark",
            Icon::Recycle => "recycle",
            Icon::Registered => "registered",
            Icon::Repeat => "repeat",
            Icon::Reply => "reply",
            Icon::ReplyAll => "reply-all",
            Icon::Republican => "republican",
            Icon::Restroom => "restroom",
            Icon::Retweet => "retweet",
            Icon::Ribbon => "ribbon",
            Icon::RightFromBracket => "right-from-bracket",
            Icon::RightLeft => "right-left",
            Icon::RightLong => "right-long",
            Icon::RightToBracket => "right-to-bracket",
            Icon::Ring => "ring",
            Icon::Road => "road",
            Icon::RoadBarrier => "road-barrier",
            Icon::RoadBridge => "road-bridge",
            Icon::RoadCircleCheck => "road-circle-check",
            Icon::RoadCircleExclamation => "road-circle-exclamation",
            Icon::RoadCircleXmark => "road-circle-xmark",
            Icon::RoadLock => "road-lock",
            Icon::RoadSpikes => "road-spikes",
            Icon::Robot => "robot",
            Icon::Rocket => "rocket",
            Icon::Rotate => "rotate",
            Icon::RotateLeft => "rotate-left",
            Icon::RotateRight => "rotate-right",
            Icon::Route => "route",
            Icon::Rss => "rss",
            Icon::RubleSign => "ruble-sign",
            Icon::Rug => "rug",
            Icon::Ruler => "ruler",
            Icon::RulerCombined => "ruler-combined",
            Icon::RulerHorizontal => "ruler-horizontal",
            Icon::RulerVertical => "ruler-vertical",
            Icon::RupeeSign => "rupee-sign",
            Icon::RupiahSign => "rupiah-sign",
            Icon::S => "s",
            Icon::SackDollar => "sack-dollar",
            Icon::SackXmark => "sack-xmark",
            Icon::Sailboat => "sailboat",
            Icon::Satellite => "satellite",
            Icon::SatelliteDish => "satellite-dish",
            Icon::ScaleBalanced => "scale-balanced",
            Icon::ScaleUnbalanced => "scale-unbalanced",
            Icon::ScaleUnbalancedFlip => "scale-unbalanced-flip",
            Icon::School => "school",
            Icon::SchoolCircleCheck => "school-circle-check",
            Icon::SchoolCircleExclamation => "school-circle-exclamation",
            Icon::SchoolCircleXmark => "school-circle-xmark",
            Icon::SchoolFlag => "school-flag",
            Icon::SchoolLock => "school-lock",
            Icon::Scissors => "scissors",
            Icon::Screwdriver => "screwdriver",
            Icon::ScrewdriverWrench => "screwdriver-wrench",
            Icon::Scroll => "scroll",
            Icon::ScrollTorah => "scroll-torah",
            Icon::SdCard => "sd-card",
            Icon::Section => "section",
            Icon::Seedling => "seedling",
            Icon::Server => "server",
            Icon::Shapes => "shapes",
            Icon::Share => "share",
            Icon::ShareFromSquare => "share-from-square",
            Icon::ShareNodes => "share-nodes",
            Icon::SheetPlastic => "sheet-plastic",
            Icon::ShekelSign => "shekel-sign",
            Icon::Shield => "shield",
            Icon::ShieldCat => "shield-cat",
            Icon::ShieldDog => "shield-dog",
            Icon::ShieldHalved => "shield-halved",
            Icon::ShieldHeart => "shield-heart",
            Icon::ShieldVirus => "shield-virus",
            Icon::Ship => "ship",
            Icon::Shirt => "shirt",
            Icon::ShoePrints => "shoe-prints",
            Icon::Shop => "shop",
            Icon::ShopLock => "shop-lock",
            Icon::ShopSlash => "shop-slash",
            Icon::Shopify => "shopify",
            Icon::Shower => "shower",
            Icon::Shrimp => "shrimp",
            Icon::Shuffle => "shuffle",
            Icon::ShuttleSpace => "shuttle-space",
            Icon::SignHanging => "sign-hanging",
            Icon::Signal => "signal",
            Icon::Signature => "signature",
            Icon::SignsPost => "signs-post",
            Icon::SimCard => "sim-card",
            Icon::Sink => "sink",
            Icon::Sitemap => "sitemap",
            Icon::Skull => "skull",
            Icon::SkullCrossbones => "skull-crossbones",
            Icon::Slack => "slack",
            Icon::Slash => "slash",
            Icon::Sleigh => "sleigh",
            Icon::Sliders => "sliders",
            Icon::Smog => "smog",
            Icon::Smoking => "smoking",
            Icon::Snowflake => "snowflake",
            Icon::Snowman => "snowman",
            Icon::Snowplow => "snowplow",
            Icon::Soap => "soap",
            Icon::Socks => "socks",
            Icon::SolarPanel => "solar-panel",
            Icon::Sort => "sort",
            Icon::SortDown => "sort-down",
            Icon::SortUp => "sort-up",
            Icon::Soundcloud => "soundcloud",
            Icon::Spa => "spa",
            Icon::SpaceAwesome => "space-awesome",
            Icon::SpaghettiMonsterFlying => "spaghetti-monster-flying",
            Icon::SpellCheck => "spell-check",
            Icon::Spider => "spider",
            Icon::Spinner => "spinner",
            Icon::Splotch => "splotch",
            Icon::Spoon => "spoon",
            Icon::Spotify => "spotify",
            Icon::SprayCan => "spray-can",
            Icon::SprayCanSparkles => "spray-can-sparkles",
            Icon::Square => "square",
            Icon::SquareArrowUpRight => "square-arrow-up-right",
            Icon::SquareCaretDown => "square-caret-down",
            Icon::SquareCaretLeft => "square-caret-left",
            Icon::SquareCaretRight => "square-caret-right",
            Icon::SquareCaretUp => "square-caret-up",
            Icon::SquareCheck => "square-check",
            Icon::SquareEnvelope => "square-envelope",
            Icon::SquareFull => "square-full",
            Icon::SquareH => "square-h",
            Icon::SquareMinus => "square-minus",
            Icon::SquareNfi => "square-nfi",
            Icon::SquareParking => "square-parking",
            Icon::SquarePen => "square-pen",
            Icon::SquarePersonConfined => "square-person-confined",
            Icon::SquarePhone => "square-phone",
            Icon::SquarePhoneFlip => "square-phone-flip",
            Icon::SquarePlus => "square-plus",
            Icon::SquarePollHorizontal => "square-poll-horizontal",
            Icon::SquarePollVertical => "square-poll-vertical",
            Icon::SquareRootVariable => "square-root-variable",
            Icon::SquareRss => "square-rss",
            Icon::SquareShareNodes => "square-share-nodes",
            Icon::SquareSteam => "square-steam",
            Icon::SquareUpRight => "square-up-right",
            Icon::SquareVirus => "square-virus",
            Icon::SquareXmark => "square-xmark",
            Icon::Squarespace => "squarespace",
            Icon::StackOverflow => "stack-overflow",
            Icon::StaffSnake => "staff-snake",
            Icon::Stairs => "stairs",
            Icon::Stamp => "stamp",
            Icon::Stapler => "stapler",
            Icon::Star => "star",
            Icon::StarAndCrescent => "star-and-crescent",
            Icon::StarHalf => "star-half",
            Icon::StarHalfStroke => "star-half-stroke",
            Icon::StarOfDavid => "star-of-david",
            Icon::StarOfLife => "star-of-life",
            Icon::Steam => "steam",
            Icon::SteamSymbol => "steam-symbol",
            Icon::SterlingSign => "sterling-sign",
            Icon::Stethoscope => "stethoscope",
            Icon::Stop => "stop",
            Icon::Stopwatch => "stopwatch",
            Icon::Stopwatch20 => "stopwatch-20",
            Icon::Store => "store",
            Icon::StoreSlash => "store-slash",
            Icon::StreetView => "street-view",
            Icon::Strikethrough => "strikethrough",
            Icon::Stripe => "stripe",
            Icon::StripeS => "stripe-s",
            Icon::Stroopwafel => "stroopwafel",
            Icon::Subscript => "subscript",
            Icon::Suitcase => "suitcase",
            Icon::SuitcaseMedical => "suitcase-medical",
            Icon::SuitcaseRolling => "suitcase-rolling",
            Icon::Sun => "sun",
            Icon::SunPlantWilt => "sun-plant-wilt",
            Icon::Superscript => "superscript",
            Icon::Swatchbook => "swatchbook",
            Icon::Synagogue => "synagogue",
            Icon::Syringe => "syringe",
            Icon::T => "t",
            Icon::Table => "table",
            Icon::TableCells => "table-cells",
            Icon::TableCellsColumnLock => "table-cells-column-lock",
            Icon::TableCellsLarge => "table-cells-large",
            Icon::TableCellsRowLock => "table-cells-row-lock",
            Icon::TableColumns => "table-columns",
            Icon::TableList => "table-list",
            Icon::TableTennisPaddleBall => "table-tennis-paddle-ball",
            Icon::Tablet => "tablet",
            Icon::TabletButton => "tablet-button",
            Icon::TabletScreenButton => "tablet-screen-button",
            Icon::Tablets => "tablets",
            Icon::TachographDigital => "tachograph-digital",
            Icon::Tag => "tag",
            Icon::Tags => "tags",
            Icon::Tape => "tape",
            Icon::Tarp => "tarp",
            Icon::TarpDroplet => "tarp-droplet",
            Icon::Taxi => "taxi",
            Icon::Teeth => "teeth",
            Icon::TeethOpen => "teeth-open",
            Icon::TemperatureArrowDown => "temperature-arrow-down",
            Icon::TemperatureArrowUp => "temperature-arrow-up",
            Icon::TemperatureEmpty => "temperature-empty",
            Icon::TemperatureFull => "temperature-full",
            Icon::TemperatureHalf => "temperature-half",
            Icon::TemperatureHigh => "temperature-high",
            Icon::TemperatureLow => "temperature-low",
            Icon::TemperatureQuarter => "temperature-quarter",
            Icon::TemperatureThreeQuarters => "temperature-three-quarters",
            Icon::TengeSign => "tenge-sign",
            Icon::Tent => "tent",
            Icon::TentArrowDownToLine => "tent-arrow-down-to-line",
            Icon::TentArrowLeftRight => "tent-arrow-left-right",
            Icon::TentArrowTurnLeft => "tent-arrow-turn-left",
            Icon::TentArrowsDown => "tent-arrows-down",
            Icon::Tents => "tents",
            Icon::Terminal => "terminal",
            Icon::TextHeight => "text-height",
            Icon::TextSlash => "text-slash",
            Icon::TextWidth => "text-width",
            Icon::Thermometer => "thermometer",
            Icon::ThumbsDown => "thumbs-down",
            Icon::ThumbsUp => "thumbs-up",
            Icon::Thumbtack => "thumbtack",
            Icon::Ticket => "ticket",
            Icon::TicketSimple => "ticket-simple",
            Icon::Tiktok => "tiktok",
            Icon::Timeline => "timeline",
            Icon::ToggleOff => "toggle-off",
            Icon::ToggleOn => "toggle-on",
            Icon::Toilet => "toilet",
            Icon::ToiletPaper => "toilet-paper",
            Icon::ToiletPaperSlash => "toilet-paper-slash",
            Icon::ToiletPortable => "toilet-portable",
            Icon::ToiletsPortable => "toilets-portable",
            Icon::Toolbox => "toolbox",
            Icon::Tooth => "tooth",
            Icon::ToriiGate => "torii-gate",
            Icon::Tornado => "tornado",
            Icon::TowerBroadcast => "tower-broadcast",
            Icon::TowerCell => "tower-cell",
            Icon::TowerObservation => "tower-observation",
            Icon::Tractor => "tractor",
            Icon::Trademark => "trademark",
            Icon::TrafficLight => "traffic-light",
            Icon::Trailer => "trailer",
            Icon::Train => "train",
            Icon::TrainSubway => "train-subway",
            Icon::TrainTram => "train-tram",
            Icon::Transgender => "transgender",
            Icon::Trash => "trash",
            Icon::TrashCan => "trash-can",
            Icon::Tree => "tree",
            Icon::TreeCity => "tree-city",
            Icon::TriangleExclamation => "triangle-exclamation",
            Icon::Trophy => "trophy",
            Icon::Trowel => "trowel",
            Icon::TrowelBricks => "trowel-bricks",
            Icon::Truck => "truck",
            Icon::TruckArrowRight => "truck-arrow-right",
            Icon::TruckDroplet => "truck-droplet",
            Icon::TruckFast => "truck-fast",
            Icon::TruckField => "truck-field",
            Icon::TruckFieldUn => "truck-field-un",
            Icon::TruckFront => "truck-front",
            Icon::TruckMedical => "truck-medical",
            Icon::TruckMonster => "truck-monster",
            Icon::TruckMoving => "truck-moving",
            Icon::TruckPickup => "truck-pickup",
            Icon::TruckPlane => "truck-plane",
            Icon::TruckRampBox => "truck-ramp-box",
            Icon::Tty => "tty",
            Icon::TurkishLiraSign => "turkish-lira-sign",
            Icon::TurnDown => "turn-down",
            Icon::TurnUp => "turn-up",
            Icon::Tv => "tv",
            Icon::Twitch => "twitch",
            Icon::Twitter => "twitter",
            Icon::U => "u",
            Icon::Umbrella => "umbrella",
            Icon::UmbrellaBeach => "umbrella-beach",
            Icon::Underline => "underline",
            Icon::UniversalAccess => "universal-access",
            Icon::Unlock => "unlock",
            Icon::UnlockKeyhole => "unlock-keyhole",
            Icon::Unsplash => "unsplash",
            Icon::UpDown => "up-down",
            Icon::UpDownLeftRight => "up-down-left-right",
            Icon::UpLong => "up-long",
            Icon::UpRightAndDownLeftFromCenter => "up-right-and-down-left-from-center",
            Icon::UpRightFromSquare => "up-right-from-square",
            Icon::Upload => "upload",
            Icon::User => "user",
            Icon::UserAstronaut => "user-astronaut",
            Icon::UserCheck => "user-check",
            Icon::UserClock => "user-clock",
            Icon::UserDoctor => "user-doctor",
            Icon::UserGear => "user-gear",
            Icon::UserGraduate => "user-graduate",
            Icon::UserGroup => "user-group",
            Icon::UserInjured => "user-injured",
            Icon::UserLarge => "user-large",
            Icon::UserLargeSlash => "user-large-slash",
            Icon::UserLock => "user-lock",
            Icon::UserMinus => "user-minus",
            Icon::UserNinja => "user-ninja",
            Icon::UserNurse => "user-nurse",
            Icon::UserPen => "user-pen",
            Icon::UserPlus => "user-plus",
            Icon::UserSecret => "user-secret",
            Icon::UserShield => "user-shield",
            Icon::UserSlash => "user-slash",
            Icon::UserTag => "user-tag",
            Icon::UserTie => "user-tie",
            Icon::UserXmark => "user-xmark",
            Icon::Users => "users",
            Icon::UsersBetweenLines => "users-between-lines",
            Icon::UsersGear => "users-gear",
            Icon::UsersLine => "users-line",
            Icon::UsersRays => "users-rays",
            Icon::UsersRectangle => "users-rectangle",
            Icon::UsersSlash => "users-slash",
            Icon::UsersViewfinder => "users-viewfinder",
            Icon::Utensils => "utensils",
            Icon::V => "v",
            Icon::VanShuttle => "van-shuttle",
            Icon::Vault => "vault",
            Icon::VectorSquare => "vector-square",
            Icon::Venus => "venus",
            Icon::VenusDouble => "venus-double",
            Icon::VenusMars => "venus-mars",
            Icon::Vest => "vest",
            Icon::VestPatches => "vest-patches",
            Icon::Vial => "vial",
            Icon::VialCircleCheck => "vial-circle-check",
            Icon::VialVirus => "vial-virus",
            Icon::Vials => "vials",
            Icon::Video => "video",
            Icon::VideoSlash => "video-slash",
            Icon::Vihara => "vihara",
            Icon::Virus => "virus",
            Icon::VirusCovid => "virus-covid",
            Icon::VirusCovidSlash => "virus-covid-slash",
            Icon::VirusSlash => "virus-slash",
            Icon::Viruses => "viruses",
            Icon::Voicemail => "voicemail",
            Icon::Volcano => "volcano",
            Icon::Volleyball => "volleyball",
            Icon::VolumeHigh => "volume-high",
            Icon::VolumeLow => "volume-low",
            Icon::VolumeOff => "volume-off",
            Icon::VolumeXmark => "volume-xmark",
            Icon::VrCardboard => "vr-cardboard",
            Icon::W => "w",
            Icon::WalkieTalkie => "walkie-talkie",
            Icon::Wallet => "wallet",
            Icon::WandMagic => "wand-magic",
            Icon::WandMagicSparkles => "wand-magic-sparkles",
            Icon::WandSparkles => "wand-sparkles",
            Icon::Warehouse => "warehouse",
            Icon::Water => "water",
            Icon::WaterLadder => "water-ladder",
            Icon::WaveSquare => "wave-square",
            Icon::WebAwesome => "web-awesome",
            Icon::WeightHanging => "weight-hanging",
            Icon::WeightScale => "weight-scale",
            Icon::WheatAwn => "wheat-awn",
            Icon::WheatAwnCircleExclamation => "wheat-awn-circle-exclamation",
            Icon::Wheelchair => "wheelchair",
            Icon::WheelchairMove => "wheelchair-move",
            Icon::WhiskeyGlass => "whiskey-glass",
            Icon::Wifi => "wifi",
            Icon::Wind => "wind",
            Icon::WindowMaximize => "window-maximize",
            Icon::WindowMinimize => "window-minimize",
            Icon::WindowRestore => "window-restore",
            Icon::Windows => "windows",
            Icon::WineBottle => "wine-bottle",
            Icon::WineGlass => "wine-glass",
            Icon::WineGlassEmpty => "wine-glass-empty",
            Icon::WizardsOfTheCoast => "wizards-of-the-coast",
            Icon::WonSign => "won-sign",
            Icon::Wordpress => "wordpress",
            Icon::Worm => "worm",
            Icon::Wrench => "wrench",
            Icon::X => "x",
            Icon::XRay => "x-ray",
            Icon::Xbox => "xbox",
            Icon::Xmark => "xmark",
            Icon::XmarksLines => "xmarks-lines",
            Icon::Y => "y",
            Icon::YenSign => "yen-sign",
            Icon::YinYang => "yin-yang",
            Icon::Youtube => "youtube",
            Icon::Z => "z",
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Returns the description for the icon.
    pub fn description(&self) -> &str {
        match self {
            Icon::Zero => "A numeral 0, representing the number.",
            Icon::One => "A numeral 1, representing the number.",
            Icon::Two => "A numeral 2, representing the number.",
            Icon::Three => "A numeral 3, representing the number.",
            Icon::Four => "A numeral 4, representing the number.",
            Icon::Five => "A numeral 5, representing the number.",
            Icon::Six => "A numeral 6, representing the number.",
            Icon::Seven => "A numeral 7, representing the number.",
            Icon::Eight => "A numeral 8, representing the number.",
            Icon::Nine => "A numeral 9, representing the number.",
            Icon::A => "A lowercase letter 'a', representing the letter.",
            Icon::AccessibleIcon => {
                "An icon of a person in a wheelchair, indicating accessibility."
            }
            Icon::AddressBook => "A book with a person's silhouette, representing a contact list.",
            Icon::AddressCard => {
                "A card with a person's silhouette, indicating contact information."
            }
            Icon::Algolia => "The logo of Algolia, representing the search engine.",
            Icon::AlignCenter => "Text aligned to the center, representing text formatting.",
            Icon::AlignJustify => "Text justified, representing text formatting.",
            Icon::AlignLeft => "Text aligned to the left, representing text formatting.",
            Icon::AlignRight => "Text aligned to the right, representing text formatting.",
            Icon::Alipay => "The logo of Alipay, representing the payment platform.",
            Icon::AmazonPay => "The logo of Amazon Pay, representing the payment service.",
            Icon::Anchor => "An anchor, representing stability or maritime themes.",
            Icon::AnchorCircleCheck => {
                "An anchor with a circled check mark, representing stability."
            }
            Icon::AnchorCircleExclamation => {
                "An anchor with a circled exclamation mark, representing caution."
            }
            Icon::AnchorCircleXmark => "An anchor with a circled `X`, representing instability.",
            Icon::AnchorLock => "An anchor with a lock, representing stability and security.",
            Icon::Android => "The logo of Android, an operating system for mobile devices.",
            Icon::AngleDown => "A downward angle, representing direction.",
            Icon::AngleLeft => "A left angle, representing direction.",
            Icon::AngleRight => "A right angle, representing direction.",
            Icon::AngleUp => "An upward angle, representing direction.",
            Icon::AnglesDown => "Two downward angles, representing direction.",
            Icon::AnglesLeft => "Two left angles, representing direction.",
            Icon::AnglesRight => "Two right angles, representing direction.",
            Icon::AnglesUp => "Two upward angles, representing direction.",
            Icon::Ankh => "The ankh symbol, representing life in ancient Egypt.",
            Icon::Apple => "The logo of Apple, representing the tech company.",
            Icon::ApplePay => "The logo of Apple Pay, representing the payment service.",
            Icon::AppleWhole => "A whole apple, representing the fruit.",
            Icon::Archway => "An architectural arch, representing structure or gateways.",
            Icon::ArrowDown => "An arrow pointing down, representing downward direction.",
            Icon::ArrowDown19 => {
                "An arrow pointing down with numbers 1 to 9, representing sorting."
            }
            Icon::ArrowDown91 => {
                "An arrow pointing down with numbers 9 to 1, representing sorting in reverse."
            }
            Icon::ArrowDownAZ => "An arrow pointing down from A to Z, representing sorting.",
            Icon::ArrowDownLong => "A long arrow pointing down, indicating downward direction.",
            Icon::ArrowDownShortWide => {
                "A short wide arrow pointing down, indicating sorting from smallest to largest."
            }
            Icon::ArrowDownUpAcrossLine => {
                "Arrows pointing down and up across a line, representing bidirectional movement."
            }
            Icon::ArrowDownUpLock => {
                "An arrow pointing down and up with a lock, representing secure bidirectional movement."
            }
            Icon::ArrowDownWideShort => {
                "A short wide arrow pointing down, indicating sorting from largest to smallest."
            }
            Icon::ArrowDownZA => {
                "An arrow pointing down with letters Z to A, representing reverse alphabetical order."
            }
            Icon::ArrowLeft => "An arrow pointing to the left, indicating direction or back.",
            Icon::ArrowLeftLong => {
                "A long arrow pointing left, representing extended backward direction."
            }
            Icon::ArrowPointer => "An arrow pointer, representing a cursor or selection.",
            Icon::ArrowRight => "A right arrow, representing forward direction.",
            Icon::ArrowRightArrowLeft => {
                "Arrows pointing right and left, representing bidirectional movement."
            }
            Icon::ArrowRightFromBracket => "A right arrow coming from a bracket, indicating exit.",
            Icon::ArrowRightLong => {
                "A long arrow pointing right, indicating extended forward direction."
            }
            Icon::ArrowRightToBracket => "A right arrow pointing to a bracket, indicating entry.",
            Icon::ArrowRightToCity => {
                "A right arrow pointing to a city, representing urban direction."
            }
            Icon::ArrowRotateLeft => {
                "An arrow rotating to the left, indicating undo or backward movement."
            }
            Icon::ArrowRotateRight => {
                "An arrow rotating to the right, indicating redo or forward movement."
            }
            Icon::ArrowTrendDown => "A downward trending arrow, representing decline.",
            Icon::ArrowTrendUp => "An upward trending arrow, representing growth or increase.",
            Icon::ArrowTurnDown => "An arrow turning down, representing downward movement.",
            Icon::ArrowTurnUp => "An arrow turning up, representing upward movement.",
            Icon::ArrowUp => "An upward arrow, indicating upward direction or increase.",
            Icon::ArrowUp19 => "An arrow pointing up with numbers 1 to 9, representing sorting.",
            Icon::ArrowUp91 => {
                "An arrow pointing up with numbers 9 to 1, representing reverse sorting."
            }
            Icon::ArrowUpAZ => {
                "An arrow pointing up with letters A to Z, representing sorting in alphabetical order."
            }
            Icon::ArrowUpFromBracket => {
                "A bracket with an upward arrow, indicating upload or elevation."
            }
            Icon::ArrowUpFromGroundWater => {
                "A ground water pump with an arrow pointing up, indicating water extraction."
            }
            Icon::ArrowUpFromWaterPump => {
                "A water pump with an arrow pointing up, indicating water extraction."
            }
            Icon::ArrowUpLong => "A long arrow pointing up, indicating upward direction.",
            Icon::ArrowUpRightDots => {
                "An arrow pointing up and right with dots, representing movement or progression."
            }
            Icon::ArrowUpRightFromSquare => {
                "An arrow pointing up-right from a square, representing an external link."
            }
            Icon::ArrowUpShortWide => {
                "A short wide arrow pointing up, indicating sorting from smallest to largest."
            }
            Icon::ArrowUpWideShort => {
                "A short wide arrow pointing up, indicating sorting from largest to smallest."
            }
            Icon::ArrowUpZA => {
                "An arrow pointing up with letters Z to A, representing reverse alphabetical order."
            }
            Icon::ArrowsDownToLine => {
                "Arrows pointing down to a line, representing downward movement."
            }
            Icon::ArrowsDownToPeople => {
                "Arrows pointing down to people, representing distribution or allocation."
            }
            Icon::ArrowsLeftRight => {
                "Arrows pointing left and right, representing bidirectional movement."
            }
            Icon::ArrowsLeftRightToLine => {
                "Arrows pointing left and right to a line, representing directional alignment."
            }
            Icon::ArrowsRotate => "Rotating arrows, representing refresh or rotation.",
            Icon::ArrowsSpin => "Arrows in a spinning motion, representing rotation or refresh.",
            Icon::ArrowsSplitUpAndLeft => "Arrows splitting up and left, representing divergence.",
            Icon::ArrowsToCircle => "Arrows pointing to a circle, representing centralization.",
            Icon::ArrowsToDot => "Arrows pointing to a dot, representing convergence or focus.",
            Icon::ArrowsToEye => "Arrows pointing to an eye, representing focus or attention.",
            Icon::ArrowsTurnRight => {
                "A set of arrows turning right, representing a directional change."
            }
            Icon::ArrowsTurnToDots => "Arrows turning to dots, representing conversion or focus.",
            Icon::ArrowsUpDown => {
                "Arrows pointing up and down, representing bidirectional movement."
            }
            Icon::ArrowsUpDownLeftRight => {
                "Arrows pointing in all directions, indicating movement or navigation."
            }
            Icon::ArrowsUpToLine => "Arrows pointing up to a line, indicating upward movement.",
            Icon::Asterisk => "An asterisk, representing additional information or footnotes.",
            Icon::At => "The at symbol (@), representing email or social media.",
            Icon::Atom => "An atom, representing science or physics.",
            Icon::AudioDescription => {
                "A screen with sound waves, indicating audio description for the visually impaired."
            }
            Icon::AustralSign => "The symbol for the Argentine austral, indicating currency.",
            Icon::Award => "A medal, representing achievement or recognition.",
            Icon::B => "The letter \"B\", representing the alphabet.",
            Icon::Baby => "A baby face, representing an infant.",
            Icon::BabyCarriage => "A baby carriage, representing childcare or infancy.",
            Icon::Backward => "An arrow pointing left, indicating backward or rewind.",
            Icon::BackwardFast => "Fast backward arrows, representing rapid reverse.",
            Icon::BackwardStep => "A step backward symbol, representing reverse or undo.",
            Icon::Bacon => "A strip of bacon, representing food or breakfast.",
            Icon::Bacteria => "Multiple bacteria, representing microbiology.",
            Icon::Bacterium => "A bacterium, representing microbiology.",
            Icon::BagShopping => "A shopping bag, representing commerce or shopping.",
            Icon::Bahai => "The symbol for the Bahá'í Faith, representing the religion.",
            Icon::BahtSign => "The symbol for the Thai baht, indicating currency.",
            Icon::Ban => "A circle with a slash, indicating prohibition.",
            Icon::BanSmoking => "A cigarette with a ban symbol, representing no smoking.",
            Icon::Bandage => "A bandage, representing first aid or healing.",
            Icon::BangladeshiTakaSign => {
                "The symbol for the Bangladeshi taka, indicating currency."
            }
            Icon::Barcode => "A barcode, representing scanning or product identification.",
            Icon::Bars => "Three horizontal bars, indicating a menu or list.",
            Icon::BarsProgress => "Bars showing progress, representing loading or progression.",
            Icon::BarsStaggered => "Staggered bars, representing a progress indicator.",
            Icon::Baseball => "A baseball, representing the sport.",
            Icon::BaseballBatBall => "A baseball bat and ball, representing the sport.",
            Icon::BasketShopping => "A shopping basket, representing retail or groceries.",
            Icon::Basketball => "A basketball, representing the sport.",
            Icon::Bath => "A bathtub, representing bathing or bathrooms.",
            Icon::BatteryEmpty => "An empty battery, representing no power.",
            Icon::BatteryFull => "A full battery, representing full charge or power.",
            Icon::BatteryHalf => "A battery half full, representing moderate power.",
            Icon::BatteryQuarter => "A battery one-quarter full, representing low power.",
            Icon::BatteryThreeQuarters => "A battery three-quarters full, representing power.",
            Icon::Bed => "A bed, representing sleep or rest.",
            Icon::BedPulse => "A bed with a pulse line, representing healthcare or emergency.",
            Icon::BeerMugEmpty => "An empty beer mug, representing beverages.",
            Icon::Bell => "A ringing bell, indicating notifications or alerts.",
            Icon::BellConcierge => "A concierge bell, representing service or assistance.",
            Icon::BellSlash => "A bell with a slash, indicating no notifications.",
            Icon::BezierCurve => "A Bézier curve, representing vector graphics or design.",
            Icon::Bicycle => "An icon of a bicycle, representing cycling.",
            Icon::Binoculars => "A pair of binoculars, indicating search or exploration.",
            Icon::Biohazard => "A biohazard symbol, representing hazardous materials.",
            Icon::Bitcoin => {
                "The logo of Bitcoin, representing the cryptocurrency, with a black background."
            }
            Icon::BitcoinSign => "The symbol for Bitcoin, indicating cryptocurrency.",
            Icon::Blender => "A blender, representing kitchen appliances.",
            Icon::BlenderPhone => "A blender with a phone, representing multitasking or devices.",
            Icon::Blog => "A blog symbol, representing blogging or writing.",
            Icon::Bluetooth => "The logo of Bluetooth, representing the wireless technology.",
            Icon::BluetoothB => "The logo of Bluetooth B, representing the wireless technology.",
            Icon::Bold => "A bold 'B', representing bold text.",
            Icon::Bolt => "A lightning bolt, representing speed or electricity.",
            Icon::BoltLightning => "A lightning bolt, representing electricity or energy.",
            Icon::Bomb => "An icon of a bomb, representing danger or explosive action.",
            Icon::Bone => "A bone, representing the skeletal system or pet treats.",
            Icon::Bong => "A bong, representing smoking or cannabis use.",
            Icon::Book => "A book, representing reading or literature.",
            Icon::BookAtlas => "A book with maps, representing an atlas or geography.",
            Icon::BookBible => "A book representing the Bible, a holy book in Christianity.",
            Icon::BookBookmark => "A book with a bookmark, representing reading or saved pages.",
            Icon::BookJournalWhills => {
                "A book representing the Journal of the Whills from Star Wars."
            }
            Icon::BookMedical => "A medical book, representing healthcare knowledge.",
            Icon::BookOpen => "An open book, representing reading or literature.",
            Icon::BookOpenReader => {
                "An open book with a user icon, representing reading or studying."
            }
            Icon::BookQuran => "A book representing the Quran, a holy book in Islam.",
            Icon::BookSkull => "A book with a skull, representing danger or mystery.",
            Icon::BookTanakh => {
                "A book representing the Tanakh, a canonical collection in Judaism."
            }
            Icon::Bookmark => "A bookmark, indicating saved items or favorites.",
            Icon::BorderAll => "An icon representing all borders.",
            Icon::BorderNone => "A border with no lines, indicating no borders.",
            Icon::BorderTopLeft => "An icon representing the top-left border.",
            Icon::BoreHole => "A borehole, representing drilling or wells.",
            Icon::BottleDroplet => "A bottle with a droplet, representing liquid or moisture.",
            Icon::BottleWater => "A bottle of water, representing hydration.",
            Icon::BowlFood => "A bowl of food, representing dining.",
            Icon::BowlRice => "A bowl of rice, representing food.",
            Icon::BowlingBall => "A bowling ball, representing the sport.",
            Icon::Box => "A simple box, representing a container.",
            Icon::BoxArchive => "A box with files, representing storage or archiving.",
            Icon::BoxOpen => "A box that is open, representing delivery or unboxing.",
            Icon::BoxTissue => "A box of tissues, representing healthcare or hygiene.",
            Icon::BoxesPacking => "Packing boxes, representing moving or storage.",
            Icon::BoxesStacked => "Stacked boxes, representing storage or organization.",
            Icon::Braille => "Braille text, representing accessibility for the blind.",
            Icon::Brain => "A brain, representing intelligence or mental processes.",
            Icon::BrazilianRealSign => "The symbol for the Brazilian real, indicating currency.",
            Icon::BreadSlice => "A slice of bread, representing food.",
            Icon::Bridge => "A simple bridge, representing infrastructure.",
            Icon::BridgeCircleCheck => {
                "A bridge with a circled check mark, indicating an approved bridge."
            }
            Icon::BridgeCircleExclamation => {
                "A bridge with a circled exclamation mark, indicating a bridge with caution."
            }
            Icon::BridgeCircleXmark => "A bridge with a circled `X`, indicating a closed bridge.",
            Icon::BridgeLock => "A bridge with a lock, representing security.",
            Icon::BridgeWater => "A bridge over water, representing infrastructure.",
            Icon::Briefcase => "A briefcase, representing work or business.",
            Icon::BriefcaseMedical => {
                "A briefcase with a medical cross, representing medical supplies."
            }
            Icon::Broom => "A broom, representing cleaning.",
            Icon::BroomBall => "A broom with a ball, representing cleaning or a sport.",
            Icon::Brush => "A brush, representing painting or art.",
            Icon::Btc => "The logo of Bitcoin, representing the cryptocurrency.",
            Icon::Bucket => "A bucket, representing a container for liquids.",
            Icon::Bug => "A bug, representing an insect or an error in software.",
            Icon::BugSlash => "A bug with a slash, indicating no bugs.",
            Icon::Bugs => "Multiple bugs, representing software issues or pests.",
            Icon::Building => "A tall building, indicating construction or urban areas.",
            Icon::BuildingCircleArrowRight => {
                "A building with a circled arrow pointing right, representing a building exit."
            }
            Icon::BuildingCircleCheck => {
                "A building with a circled check mark, representing an approved building."
            }
            Icon::BuildingCircleExclamation => {
                "A building with a circled exclamation mark, representing a building with caution."
            }
            Icon::BuildingCircleXmark => {
                "A building with a circled `X`, representing a closed building."
            }
            Icon::BuildingColumns => {
                "A building with columns, representing classical architecture."
            }
            Icon::BuildingFlag => "A building with a flag, representing government or institution.",
            Icon::BuildingLock => "A building with a lock, representing security.",
            Icon::BuildingNgo => {
                "A building with 'NGO', representing a non-governmental organization."
            }
            Icon::BuildingShield => "A building with a shield, representing security.",
            Icon::BuildingUn => "A building with 'UN', representing the United Nations.",
            Icon::BuildingUser => {
                "A building with a user icon, representing a workplace or office."
            }
            Icon::BuildingWheat => {
                "A building with wheat, representing agriculture or agribusiness."
            }
            Icon::Bullhorn => "A bullhorn, representing announcements or public address.",
            Icon::Bullseye => "A bullseye, representing a target or goal.",
            Icon::Burger => "A burger, representing food or fast food.",
            Icon::Burst => "An explosion or burst, representing impact or energy.",
            Icon::Bus => "A bus, representing public transportation.",
            Icon::BusSimple => "A simple bus, representing public transportation.",
            Icon::BusinessTime => {
                "A briefcase with a clock, representing business hours or time management."
            }
            Icon::C => "A capital letter 'C', representing the letter.",
            Icon::CableCar => "A cable car, representing a type of public transportation.",
            Icon::CakeCandles => "A cake with candles, representing celebration or birthday.",
            Icon::Calculator => "A calculator, representing mathematical calculations.",
            Icon::Calendar => "A simple calendar, representing scheduling.",
            Icon::CalendarCheck => "A calendar with a check mark, representing a confirmed date.",
            Icon::CalendarDay => "A calendar showing a day, representing scheduling.",
            Icon::CalendarDays => "A calendar with marked days, indicating a schedule or event.",
            Icon::CalendarMinus => "A calendar with a minus sign, representing removing an event.",
            Icon::CalendarPlus => "A calendar with a plus sign, representing adding an event.",
            Icon::CalendarWeek => "A calendar with a week view, representing weekly schedule.",
            Icon::CalendarXmark => "A calendar with an `X`, representing a cancelled date.",
            Icon::Camera => "An icon of a camera, representing photography.",
            Icon::CameraRetro => "An old-fashioned camera, indicating photography or photos.",
            Icon::CameraRotate => "A camera with a rotation arrow, representing photo orientation.",
            Icon::Campground => "A campground symbol, representing camping or outdoor activities.",
            Icon::CandyCane => "A candy cane, representing Christmas or sweets.",
            Icon::Cannabis => "A cannabis leaf, representing the plant or its products.",
            Icon::Capsules => "Two capsules, representing medication or supplements.",
            Icon::Car => "An icon of a car, indicating a vehicle or transportation.",
            Icon::CarBattery => "A car battery, representing automotive power.",
            Icon::CarBurst => "A car with a burst, indicating accident or impact.",
            Icon::CarOn => "A car with a key, indicating vehicle status.",
            Icon::CarRear => "The rear view of a car, representing transportation.",
            Icon::CarSide => "A side view of a car, indicating transportation.",
            Icon::CarTunnel => "A car in a tunnel, representing travel or transportation.",
            Icon::Caravan => "A caravan, representing travel or transportation.",
            Icon::CaretDown => "A downward caret, representing dropdowns or more options.",
            Icon::CaretLeft => "A caret pointing left, indicating backward direction.",
            Icon::CaretRight => "A caret pointing right, indicating forward direction.",
            Icon::CaretUp => "An upward pointing caret, indicating expansion or scroll up.",
            Icon::Carrot => "A carrot, representing the vegetable.",
            Icon::CartArrowDown => {
                "A shopping cart with a downward arrow, representing adding to cart."
            }
            Icon::CartFlatbed => "A flatbed cart, representing transportation or logistics.",
            Icon::CartFlatbedSuitcase => {
                "A flatbed cart with a suitcase, representing luggage transport."
            }
            Icon::CartPlus => "A shopping cart with a plus sign, representing adding to cart.",
            Icon::CartShopping => "A shopping cart, representing commerce or shopping.",
            Icon::CashRegister => "A cash register, indicating point of sale or retail.",
            Icon::Cat => "A cat, representing the animal.",
            Icon::CcAmazonPay => {
                "The logo of CC Amazon Pay, representing the credit card payment service."
            }
            Icon::CcAmex => "The logo of CC Amex, representing the credit card payment service.",
            Icon::CcApplePay => {
                "The logo of CC Apple Pay, representing the credit card payment service."
            }
            Icon::CcDinersClub => {
                "The logo of CC Diners Club, representing the credit card payment service."
            }
            Icon::CcDiscover => {
                "The logo of CC Discover, representing the credit card payment service."
            }
            Icon::CcJcb => "The logo of CC JCB, representing the credit card payment service.",
            Icon::CcMastercard => "The logo of MasterCard, indicating a credit card or payment.",
            Icon::CcPaypal => {
                "The logo of CC PayPal, representing the credit card payment service."
            }
            Icon::CcStripe => {
                "The logo of CC Stripe, representing the credit card payment service."
            }
            Icon::CcVisa => "The logo of Visa credit card, indicating payment.",
            Icon::CediSign => "The symbol for the Ghanaian cedi, indicating currency.",
            Icon::CentSign => "The symbol for cent, indicating currency.",
            Icon::Certificate => "A certificate, indicating achievement or certification.",
            Icon::Chair => "A chair, representing seating or furniture.",
            Icon::Chalkboard => "A chalkboard, representing teaching or education.",
            Icon::ChalkboardUser => {
                "A chalkboard with a user icon, representing teaching or instruction."
            }
            Icon::ChampagneGlasses => "Two champagne glasses clinking, representing celebration.",
            Icon::ChargingStation => "A charging station, representing electric vehicle charging.",
            Icon::ChartArea => "An area chart, representing data trends.",
            Icon::ChartBar => "A bar chart, representing data comparison.",
            Icon::ChartColumn => "A column chart, representing data visualization.",
            Icon::ChartGantt => "A Gantt chart, representing project management.",
            Icon::ChartLine => "A line chart, representing data trends.",
            Icon::ChartPie => "A pie chart, representing data visualization.",
            Icon::ChartSimple => "A simple bar chart, representing data or statistics.",
            Icon::Check => "A check mark, symbolizing confirmation or success.",
            Icon::CheckDouble => "A double check mark, representing confirmation or approval.",
            Icon::CheckToSlot => "A checkmark entering a slot, representing verification.",
            Icon::Cheese => "A wedge of cheese, representing dairy or food.",
            Icon::Chess => "A chess piece, representing the game of chess.",
            Icon::ChessBishop => "A chess bishop, representing the game of chess.",
            Icon::ChessBoard => "A chess board, representing the game of chess.",
            Icon::ChessKing => "A chess king, representing the game of chess.",
            Icon::ChessKnight => "A chess knight, representing the game of chess.",
            Icon::ChessPawn => "A chess pawn, representing the game of chess.",
            Icon::ChessQueen => "A chess queen, representing the game of chess.",
            Icon::ChessRook => "A chess rook, representing the game of chess.",
            Icon::ChevronDown => "A downward chevron, representing a dropdown or more options.",
            Icon::ChevronLeft => "A chevron pointing left, indicating backward direction.",
            Icon::ChevronRight => "A chevron pointing right, indicating forward direction.",
            Icon::ChevronUp => {
                "A chevron pointing upwards, indicating upward movement or navigation."
            }
            Icon::Child => "A child, indicating a young person.",
            Icon::ChildCombatant => "A child holding a weapon, representing child soldiers.",
            Icon::ChildDress => "A child in a dress, representing a young girl.",
            Icon::ChildReaching => "A child reaching out, representing assistance or curiosity.",
            Icon::Children => "Two children, representing youth or family.",
            Icon::Church => "A church building, representing a place of worship.",
            Icon::Circle => "A simple circle, representing shape or completeness.",
            Icon::CircleArrowDown => {
                "A circle with an arrow pointing down, indicating downward movement."
            }
            Icon::CircleArrowLeft => {
                "A circle with an arrow pointing left, indicating backward movement."
            }
            Icon::CircleArrowRight => {
                "A circle with an arrow pointing right, indicating forward movement."
            }
            Icon::CircleArrowUp => {
                "A circle with an arrow pointing up, indicating upward movement."
            }
            Icon::CircleCheck => "A check mark inside a circle, indicating confirmation.",
            Icon::CircleChevronDown => {
                "A circle with a chevron pointing down, indicating downward direction."
            }
            Icon::CircleChevronLeft => {
                "A circle with a chevron pointing left, indicating backward direction."
            }
            Icon::CircleChevronRight => {
                "A circle with a chevron pointing right, indicating forward direction."
            }
            Icon::CircleChevronUp => {
                "A circle with a chevron pointing up, indicating upward direction."
            }
            Icon::CircleDollarToSlot => "A circle with a dollar sign and slot, indicating payment.",
            Icon::CircleDot => "A circle with a dot, indicating focus or selection.",
            Icon::CircleDown => {
                "A downward arrow inside a circle, indicating scroll down or download."
            }
            Icon::CircleExclamation => {
                "An exclamation mark inside a circle, indicating important information or alerts."
            }
            Icon::CircleH => "A circle with an 'H', representing hospital.",
            Icon::CircleHalfStroke => {
                "A half-filled circle, representing partial loading or status."
            }
            Icon::CircleInfo => "A circle with an 'i', representing information.",
            Icon::CircleLeft => "A circle with a left arrow, indicating backward direction.",
            Icon::CircleMinus => "A circle with a minus sign, indicating subtraction.",
            Icon::CircleNodes => "A circle with nodes, representing connections or network.",
            Icon::CircleNotch => {
                "A circle with a notch, representing a loading or progress indicator."
            }
            Icon::CirclePause => "A circle with a pause symbol, indicating media pause.",
            Icon::CirclePlay => "A circle with a play symbol, indicating media playback.",
            Icon::CirclePlus => "A circle with a plus sign, indicating addition.",
            Icon::CircleQuestion => "A circle with a question mark, indicating inquiry.",
            Icon::CircleRadiation => "A circle with a radiation symbol, indicating hazard.",
            Icon::CircleRight => "A circle with a right arrow, indicating forward direction.",
            Icon::CircleStop => "A circle with a stop symbol, indicating cessation.",
            Icon::CircleUp => "An upward arrow inside a circle, indicating scroll up or upload.",
            Icon::CircleUser => "A user icon inside a circle, indicating a user profile.",
            Icon::CircleXmark => "A circled `X` mark, indicating closure or deletion.",
            Icon::City => "A skyline of buildings, indicating an urban area or city.",
            Icon::Clapperboard => "A clapperboard, representing filmmaking or production.",
            Icon::Clipboard => "A clipboard, representing note-taking or data recording.",
            Icon::ClipboardCheck => "A clipboard with a check mark, representing completed tasks.",
            Icon::ClipboardList => "A clipboard with a list, representing tasks or notes.",
            Icon::ClipboardQuestion => {
                "A clipboard with a question mark, representing inquiry or uncertainty."
            }
            Icon::ClipboardUser => "A clipboard with a user icon, representing user data or forms.",
            Icon::Clock => "A clock face, indicating time.",
            Icon::ClockRotateLeft => {
                "A clock with an arrow rotating left, representing time reversal."
            }
            Icon::Clone => "Two overlapping squares, indicating duplication or cloning.",
            Icon::ClosedCaptioning => {
                "A closed captioning symbol, representing subtitles or accessibility."
            }
            Icon::Cloud => "A cloud, representing cloud storage or weather.",
            Icon::CloudArrowDown => "A cloud with a downward arrow, representing cloud download.",
            Icon::CloudArrowUp => "A cloud with an upward arrow, indicating upload to the cloud.",
            Icon::CloudBolt => "A cloud with a lightning bolt, representing a thunderstorm.",
            Icon::CloudMeatball => "A cloud with meatballs, representing food or weather.",
            Icon::CloudMoon => "A cloud with a moon, representing partly cloudy night.",
            Icon::CloudMoonRain => "A cloud with a moon and rain, representing nighttime rain.",
            Icon::CloudRain => "A cloud with rain, representing weather or precipitation.",
            Icon::CloudShowersHeavy => "A cloud with heavy rain, representing a downpour.",
            Icon::CloudShowersWater => "A cloud with water droplets, representing rain.",
            Icon::CloudSun => "A cloud with a sun, representing partly cloudy weather.",
            Icon::CloudSunRain => "A cloud with sun and rain, representing mixed weather.",
            Icon::Cloudflare => {
                "The logo of Cloudflare, representing the web infrastructure company."
            }
            Icon::Clover => "A clover, representing luck or St. Patrick's Day.",
            Icon::Code => "An icon representing coding or programming.",
            Icon::CodeBranch => "A branch in code, indicating version control or branching.",
            Icon::CodeCommit => "A check mark, representing a code commit.",
            Icon::CodeCompare => {
                "Two pieces of code being compared, indicating code review or comparison."
            }
            Icon::CodeFork => "A forked path, representing branching in code.",
            Icon::CodeMerge => "A symbol representing code merging.",
            Icon::CodePullRequest => "A symbol representing a pull request in code versioning.",
            Icon::Codepen => {
                "The logo of CodePen, a social development environment for front-end designers and developers."
            }
            Icon::Coins => "Coins, representing money or currency.",
            Icon::ColonSign => "A colon symbol, representing punctuation or separation.",
            Icon::Comment => "A speech bubble, indicating comments or communication.",
            Icon::CommentDollar => {
                "A speech bubble with a dollar sign, representing financial comments."
            }
            Icon::CommentDots => {
                "A speech bubble with dots, representing comments or conversation."
            }
            Icon::CommentMedical => {
                "A speech bubble with a medical cross, representing medical communication."
            }
            Icon::CommentSlash => "A speech bubble with a slash, indicating no comments.",
            Icon::CommentSms => "A speech bubble with \"SMS\", representing text messaging.",
            Icon::Comments => "Multiple speech bubbles, indicating conversation or comments.",
            Icon::CommentsDollar => {
                "A speech bubble with a dollar sign, representing financial discussions."
            }
            Icon::CompactDisc => "A compact disc, representing media storage.",
            Icon::Compass => "A compass, representing navigation or direction.",
            Icon::CompassDrafting => "A drafting compass, representing design or architecture.",
            Icon::Compress => "A compress icon, representing minimization.",
            Icon::Computer => "A desktop computer, representing computing or technology.",
            Icon::ComputerMouse => "A computer mouse, representing input device.",
            Icon::Cookie => "A cookie, representing snacks or website tracking.",
            Icon::CookieBite => "A bitten cookie, representing snacks or desserts.",
            Icon::Copy => "Two overlapping documents, indicating copying.",
            Icon::Copyright => "A circled 'C', indicating copyright protection.",
            Icon::Couch => "A couch, representing furniture or relaxation.",
            Icon::Cow => "A cow, representing the animal.",
            Icon::CreditCard => "A credit card, indicating payment or financial transactions.",
            Icon::CriticalRole => "The logo of Critical Role, representing the web series.",
            Icon::Crop => "An image crop icon, representing editing.",
            Icon::CropSimple => "A simple crop icon, representing image cropping.",
            Icon::Cross => "A cross, representing religion or medical aid.",
            Icon::Crosshairs => "A crosshair, indicating targeting or precision.",
            Icon::Crow => "A crow, representing the bird.",
            Icon::Crown => "A crown, representing royalty or achievement.",
            Icon::Crutch => "A crutch, representing injury support.",
            Icon::CruzeiroSign => "The symbol for the Brazilian cruzeiro, indicating currency.",
            Icon::Cube => "A 3D cube, representing geometry or structure.",
            Icon::Cubes => "Multiple cubes, representing 3D objects.",
            Icon::CubesStacked => "Stacked cubes, representing building blocks.",
            Icon::D => "A capital letter 'D', representing the letter.",
            Icon::DAndD => {
                "The logo of Dungeons & Dragons, representing the tabletop role-playing game."
            }
            Icon::DAndDBeyond => {
                "The logo of D&D Beyond, representing the Dungeons & Dragons toolset."
            }
            Icon::Database => "A stack of disks, representing a database.",
            Icon::DeleteLeft => "An arrow pointing left with a line, representing backspace.",
            Icon::Democrat => "The logo of the Democratic Party, representing the political party.",
            Icon::Desktop => "A desktop computer, indicating computing or work.",
            Icon::Dharmachakra => "The dharma wheel, representing Buddhism.",
            Icon::DiagramNext => "A diagram showing the next step, representing progression.",
            Icon::DiagramPredecessor => "A diagram showing predecessors, representing planning.",
            Icon::DiagramProject => "A diagram showing a project, representing planning.",
            Icon::DiagramSuccessor => "A diagram showing successors, representing progression.",
            Icon::Diamond => "A diamond, representing luxury or value.",
            Icon::DiamondTurnRight => {
                "A diamond turned to the right, representing geometric shapes."
            }
            Icon::Dice => "A pair of dice, representing games or chance.",
            Icon::DiceD20 => "A 20-sided die, representing tabletop gaming.",
            Icon::DiceD6 => "A six-sided die, representing gaming or chance.",
            Icon::DiceFive => "Two dice showing five, representing chance or gaming.",
            Icon::DiceFour => "Two dice showing four, representing chance or gaming.",
            Icon::DiceOne => "Two dice showing one, representing chance or gaming.",
            Icon::DiceSix => "Two dice showing six, representing chance or gaming.",
            Icon::DiceThree => "Two dice showing three, representing chance or gaming.",
            Icon::DiceTwo => "Two dice showing two, representing chance or gaming.",
            Icon::Discord => "The logo of Discord, a chat and communication platform for gamers.",
            Icon::Disease => "A virus, representing illness.",
            Icon::Display => "A computer display, representing screens or monitors.",
            Icon::Divide => "A division sign, representing mathematical operations.",
            Icon::Dna => "A DNA strand, representing genetics.",
            Icon::Docker => "The logo of Docker, a platform for containerized applications.",
            Icon::Dog => "A dog, representing the animal.",
            Icon::DollarSign => "A dollar sign, indicating currency or money.",
            Icon::Dolly => "A dolly, representing transport or moving.",
            Icon::DongSign => "The symbol for the Vietnamese dong, indicating currency.",
            Icon::DoorClosed => "A closed door, representing privacy or security.",
            Icon::DoorOpen => "An open door, indicating entry or exit.",
            Icon::Dove => "A dove, representing peace.",
            Icon::DownLeftAndUpRightToCenter => {
                "Arrows pointing down-left and up-right to a center, representing convergence."
            }
            Icon::DownLong => "A long arrow pointing down, representing downward direction.",
            Icon::Download => "A downward arrow, typically used to indicate download actions.",
            Icon::Dragon => "A dragon, representing mythical creatures or fantasy.",
            Icon::DrawPolygon => "A polygon, representing geometric shapes.",
            Icon::Dribbble => "The logo of Dribbble, a platform for showcasing design work.",
            Icon::Dropbox => "The logo of Dropbox, a cloud storage service.",
            Icon::Droplet => "A droplet of water, representing liquid or fluidity.",
            Icon::DropletSlash => "A droplet with a slash, representing no water.",
            Icon::Drum => "A drum, representing music.",
            Icon::DrumSteelpan => "A steelpan drum, representing music.",
            Icon::DrumstickBite => "A drumstick with a bite, representing food.",
            Icon::Dumbbell => "A dumbbell, representing fitness or weightlifting.",
            Icon::Dumpster => "A dumpster, representing waste disposal.",
            Icon::DumpsterFire => "A dumpster on fire, representing chaos or disaster.",
            Icon::Dungeon => "A dungeon, representing a prison or game environment.",
            Icon::E => "The letter \"E\", representing the alphabet.",
            Icon::EarDeaf => "An ear with a slash, representing hearing impairment.",
            Icon::EarListen => "An ear with sound waves, representing listening.",
            Icon::EarthAfrica => "A globe focusing on Africa, representing global presence.",
            Icon::EarthAmericas => "A globe focusing on the Americas, representing global reach.",
            Icon::EarthAsia => "A globe focusing on Asia, representing global presence.",
            Icon::EarthEurope => "A globe focusing on Europe, representing global presence.",
            Icon::EarthOceania => "A globe focusing on Oceania, representing global presence.",
            Icon::Egg => "An egg, representing food or Easter.",
            Icon::Eject => "An eject button, representing removal.",
            Icon::Elevator => "An elevator, representing vertical transportation.",
            Icon::Ellipsis => "A horizontal ellipsis, representing more options.",
            Icon::EllipsisVertical => "A vertical ellipsis, representing more options.",
            Icon::Envelope => "A closed envelope, representing email or messages.",
            Icon::EnvelopeCircleCheck => {
                "An envelope with a circled check, representing approved mail."
            }
            Icon::EnvelopeOpen => "An open envelope, representing received message.",
            Icon::EnvelopeOpenText => "An open envelope with text, representing received message.",
            Icon::EnvelopesBulk => "Multiple envelopes, representing bulk mail.",
            Icon::Equals => "An equals sign, representing equality.",
            Icon::Eraser => "An eraser, representing correction or deletion.",
            Icon::Ethereum => "The logo of Ethereum, representing the cryptocurrency.",
            Icon::Ethernet => "An Ethernet port, representing network connectivity.",
            Icon::EuroSign => "The symbol for the euro, indicating currency.",
            Icon::Exclamation => "A large exclamation mark, indicating importance or alerts.",
            Icon::Expand => "An outward pointing arrows from a box, indicating expansion.",
            Icon::Explosion => "An explosion, representing blast or impact.",
            Icon::Eye => "An eye, indicating visibility or views.",
            Icon::EyeDropper => "An eyedropper, representing precision or medical use.",
            Icon::EyeLowVision => "An eye with low vision, representing visual impairment.",
            Icon::EyeSlash => {
                "An eye with a slash through it, indicating hidden or invisible content."
            }
            Icon::F => "A capital letter 'F', representing the letter.",
            Icon::FaceAngry => "An angry face, representing anger.",
            Icon::FaceDizzy => "A dizzy face, representing confusion.",
            Icon::FaceFlushed => "A flushed face, representing embarrassment.",
            Icon::FaceFrown => "A frowning face, representing sadness.",
            Icon::FaceFrownOpen => "A frowning face with open mouth, representing sadness.",
            Icon::FaceGrimace => "A grimacing face, representing discomfort.",
            Icon::FaceGrin => "A grinning face, representing happiness.",
            Icon::FaceGrinBeam => "A grinning face with beams, representing joy.",
            Icon::FaceGrinBeamSweat => "A grinning face with beam and sweat, representing relief.",
            Icon::FaceGrinHearts => "A grinning face with hearts, representing love.",
            Icon::FaceGrinSquint => "A grinning face with squinted eyes, representing humor.",
            Icon::FaceGrinSquintTears => {
                "A grinning face with squinting eyes and tears, representing laughter."
            }
            Icon::FaceGrinStars => "A grinning face with stars, representing excitement.",
            Icon::FaceGrinTears => "A grinning face with tears, representing laughter.",
            Icon::FaceGrinTongue => "A grinning face with tongue out, representing playfulness.",
            Icon::FaceGrinTongueSquint => {
                "A grinning face with tongue out and squinted eyes, representing silliness."
            }
            Icon::FaceGrinTongueWink => {
                "A grinning face with tongue out and wink, representing silliness."
            }
            Icon::FaceGrinWide => "A wide grinning face, representing happiness.",
            Icon::FaceGrinWink => "A grinning face with a wink, representing playfulness.",
            Icon::FaceKiss => "A kissing face, representing affection.",
            Icon::FaceKissBeam => "A kissing face with beams, representing love.",
            Icon::FaceKissWinkHeart => {
                "A kissing face with a wink and heart, representing affection."
            }
            Icon::FaceLaugh => "A laughing face, representing humor.",
            Icon::FaceLaughBeam => "A laughing face with beams, representing joy.",
            Icon::FaceLaughSquint => "A laughing face with squinted eyes, representing humor.",
            Icon::FaceLaughWink => "A laughing face with a wink, representing humor.",
            Icon::FaceMeh => "A meh face, representing indifference.",
            Icon::FaceMehBlank => "A blank face, representing indifference.",
            Icon::FaceRollingEyes => "A face with rolling eyes, representing annoyance.",
            Icon::FaceSadCry => "A crying face, representing sadness or crying.",
            Icon::FaceSadTear => "A sad face with a tear, representing sadness or crying.",
            Icon::FaceSmile => "A smiling face, indicating happiness or friendliness.",
            Icon::FaceSmileBeam => "A smiling face with beams, representing joy.",
            Icon::FaceSmileWink => {
                "A smiling face with a wink, representing happiness or playfulness."
            }
            Icon::FaceSurprise => "A surprised face, representing shock.",
            Icon::FaceTired => "A tired face, representing fatigue.",
            Icon::Facebook => "The logo of Facebook, representing the social media platform.",
            Icon::Fan => "A fan, representing cooling or ventilation.",
            Icon::FantasyFlightGames => {
                "The logo of Fantasy Flight Games, representing the game publisher."
            }
            Icon::Faucet => "A faucet, representing plumbing or water.",
            Icon::FaucetDrip => "A faucet with a drip, representing water or plumbing.",
            Icon::Fax => "A fax machine, representing facsimile transmission.",
            Icon::Feather => "A feather, indicating lightness or writing.",
            Icon::FeatherPointed => "A pointed feather, representing writing or quill.",
            Icon::Ferry => "A ferry boat, representing water transportation.",
            Icon::Figma => "The logo of Figma, a design and prototyping tool.",
            Icon::File => "A simple document, indicating a file or document.",
            Icon::FileArrowDown => "A file with a downward arrow, representing file download.",
            Icon::FileArrowUp => "A file with an upward arrow, representing file upload.",
            Icon::FileAudio => "A file with an audio symbol, representing audio files.",
            Icon::FileCircleCheck => "A file with a circled check, representing approved files.",
            Icon::FileCircleExclamation => {
                "A file with a circled exclamation mark, representing important files."
            }
            Icon::FileCircleMinus => "A file with a circled minus, representing removed files.",
            Icon::FileCirclePlus => "A file with a circled plus, representing added files.",
            Icon::FileCircleQuestion => {
                "A file with a circled question mark, representing unknown files."
            }
            Icon::FileCircleXmark => "A file with a circled `X`, representing deleted files.",
            Icon::FileCode => "A file with code, representing programming files.",
            Icon::FileContract => {
                "A document with a signature line, indicating a contract or agreement."
            }
            Icon::FileCsv => "A file with CSV text, representing CSV files.",
            Icon::FileExcel => "A document with the Excel logo, indicating a spreadsheet file.",
            Icon::FileExport => "A document with an arrow pointing out, indicating file export.",
            Icon::FileImage => "A file with an image, representing image files.",
            Icon::FileImport => "A document with an arrow pointing in, indicating file import.",
            Icon::FileInvoice => "A document with an invoice, indicating billing or payments.",
            Icon::FileInvoiceDollar => {
                "A file with a dollar sign, representing financial documents."
            }
            Icon::FileLines => "A file with lines, representing documents.",
            Icon::FileMedical => "A file with a medical symbol, representing medical records.",
            Icon::FilePdf => "A file with a PDF symbol, representing a document.",
            Icon::FilePen => "A file with a pen, representing editable documents.",
            Icon::FilePowerpoint => "A file with a PowerPoint symbol, representing presentations.",
            Icon::FilePrescription => {
                "A file with a prescription symbol, representing medical records."
            }
            Icon::FileShield => "A file with a shield, representing secure documents.",
            Icon::FileSignature => "A file with a signature, representing signed documents.",
            Icon::FileVideo => "A file with a video symbol, representing video files.",
            Icon::FileWaveform => "A file with a waveform, representing audio files.",
            Icon::FileWord => "A file with a Word symbol, representing a document.",
            Icon::FileZipper => "A file with a zipper, representing compressed files.",
            Icon::Fill => "A paint bucket pouring, representing filling or color.",
            Icon::FillDrip => "A paint bucket dripping, representing paint or color fill.",
            Icon::Film => "A strip of film, representing movies or filming.",
            Icon::Filter => "A funnel filter, representing filtration or sorting.",
            Icon::FilterCircleDollar => {
                "A filter with a circled dollar sign, representing financial filtering."
            }
            Icon::FilterCircleXmark => "A filter with a circled X, representing filter removal.",
            Icon::Fingerprint => "A fingerprint, representing identity or security.",
            Icon::Fire => "A flame, representing fire or heat.",
            Icon::FireBurner => "A fire burner, representing heating or cooking.",
            Icon::FireExtinguisher => "A fire extinguisher, representing safety equipment.",
            Icon::FireFlameCurved => "A curved flame, representing fire.",
            Icon::FireFlameSimple => "A simple flame, representing fire or heat.",
            Icon::Fish => "A fish, representing the animal or aquatic life.",
            Icon::FishFins => "A fish with fins, representing the animal or swimming.",
            Icon::Flag => "A flag, indicating a nation or marking something important.",
            Icon::FlagCheckered => "A checkered flag, representing racing or completion.",
            Icon::FlagUsa => "The flag of the USA, representing the United States of America.",
            Icon::Flask => "A laboratory flask, representing science or experimentation.",
            Icon::FlaskVial => "A flask and vial, representing science or experimentation.",
            Icon::FloppyDisk => "A floppy disk, representing data storage.",
            Icon::FlorinSign => "The symbol for the florin, indicating currency.",
            Icon::Folder => "A folder, representing a collection of documents or files.",
            Icon::FolderClosed => "A closed folder, representing file storage.",
            Icon::FolderMinus => "A folder with a minus sign, indicating removing files.",
            Icon::FolderOpen => "An open folder, indicating accessible files or documents.",
            Icon::FolderPlus => "A folder with a plus sign, indicating adding files.",
            Icon::FolderTree => "A folder with a tree structure, representing organization.",
            Icon::Font => "A capital letter 'A', representing typography or fonts.",
            Icon::FontAwesome => "The logo of Font Awesome, representing the icon set.",
            Icon::Football => "A football, representing the sport.",
            Icon::Forward => "An arrow pointing right, indicating forward or next.",
            Icon::ForwardFast => "Two arrows pointing forward, representing fast forward.",
            Icon::ForwardStep => {
                "An arrow pointing forward with a vertical line, indicating step forward."
            }
            Icon::FrancSign => "The symbol for the franc, indicating currency.",
            Icon::Frog => "A frog, representing the animal.",
            Icon::Futbol => "A soccer ball, representing the sport of soccer.",
            Icon::G => "A capital letter 'G', representing the letter.",
            Icon::GalacticRepublic => {
                "The logo of the Galactic Republic, representing the Star Wars faction."
            }
            Icon::GalacticSenate => {
                "The logo of the Galactic Senate, representing the Star Wars faction."
            }
            Icon::Gamepad => "A video game controller, representing gaming.",
            Icon::GasPump => "A gas pump, representing fuel or energy.",
            Icon::Gauge => "A speedometer or gauge, representing measurement or speed.",
            Icon::GaugeHigh => "A high gauge, representing high level or measurement.",
            Icon::GaugeSimple => "A simple gauge, representing measurement.",
            Icon::GaugeSimpleHigh => "A gauge with a high reading, representing high measurement.",
            Icon::Gavel => "A gavel, representing law or auctions.",
            Icon::Gear => "A gear, representing settings or machinery.",
            Icon::Gears => "Multiple gears, representing settings or machinery.",
            Icon::Gem => "A gemstone, representing jewelry or value.",
            Icon::Genderless => "A genderless symbol, representing gender neutrality.",
            Icon::Gg => "The logo of GG, representing good game.",
            Icon::GgCircle => "A circle with \"GG\", representing good game.",
            Icon::Ghost => "A ghost, often used for spooky or playful themes.",
            Icon::Gift => "A wrapped gift box, representing presents or surprises.",
            Icon::Gifts => "Two wrapped gifts, representing presents or surprises.",
            Icon::Github => "The logo of GitHub, representing the code hosting platform.",
            Icon::GlassWater => "A glass of water, representing hydration.",
            Icon::GlassWaterDroplet => "A glass of water with a droplet, representing hydration.",
            Icon::Glasses => "A pair of glasses, representing vision or eyewear.",
            Icon::Globe => "A globe, representing the world or global reach.",
            Icon::GolfBallTee => "A golf ball on a tee, representing golf.",
            Icon::Google => "The logo of Google, a popular search engine.",
            Icon::GooglePay => "The logo of Google Pay, representing the payment service.",
            Icon::GoogleWallet => "The logo of Google Wallet, representing the payment service.",
            Icon::Gopuram => "A gopuram, representing a Hindu temple tower.",
            Icon::GraduationCap => "A graduation cap, representing education or graduation.",
            Icon::GreaterThan => "A greater than sign, representing mathematical operations.",
            Icon::GreaterThanEqual => {
                "A greater than or equal sign, representing mathematical operations."
            }
            Icon::Grip => "Dots indicating grip or draggable interface.",
            Icon::GripLines => "Horizontal lines indicating grip or draggable interface.",
            Icon::GripLinesVertical => "Vertical lines indicating grip or draggable interface.",
            Icon::GripVertical => "Vertical dots indicating grip or draggable interface.",
            Icon::GroupArrowsRotate => {
                "Multiple arrows rotating around a group, indicating movement or rotation."
            }
            Icon::GuaraniSign => "The symbol for the Paraguayan guaraní, indicating currency.",
            Icon::Guitar => "A guitar, representing music.",
            Icon::Gun => "A gun, representing firearms.",
            Icon::H => "A capital letter 'H', representing the letter.",
            Icon::Hammer => "A hammer, representing tools or construction.",
            Icon::Hamsa => "A hamsa hand, representing protection or luck.",
            Icon::Hand => "A raised hand, indicating a stop or request for attention.",
            Icon::HandBackFist => "A hand with a fist facing backwards, representing strength.",
            Icon::HandDots => "A hand with dots, representing tactile or touch.",
            Icon::HandFist => "A fist, representing strength or power.",
            Icon::HandHolding => "A hand holding something, representing support.",
            Icon::HandHoldingDollar => {
                "A hand holding a dollar sign, representing financial support."
            }
            Icon::HandHoldingDroplet => "A hand holding a droplet, representing water or liquid.",
            Icon::HandHoldingHand => {
                "A hand holding another hand, representing support or assistance."
            }
            Icon::HandHoldingHeart => "A hand holding a heart, symbolizing charity or care.",
            Icon::HandHoldingMedical => {
                "A hand holding a medical symbol, representing healthcare or support."
            }
            Icon::HandLizard => "A hand making a lizard gesture, representing the animal.",
            Icon::HandMiddleFinger => {
                "A hand making the middle finger gesture, representing rudeness."
            }
            Icon::HandPeace => "A hand making a peace sign, representing peace or victory.",
            Icon::HandPointDown => "A hand pointing down, representing direction.",
            Icon::HandPointLeft => "A hand pointing to the left, representing direction.",
            Icon::HandPointRight => "A hand pointing to the right, representing direction.",
            Icon::HandPointUp => "A hand pointing upwards, indicating direction or emphasis.",
            Icon::HandPointer => "A hand pointer, representing selection or clicking.",
            Icon::HandScissors => {
                "A hand making a scissors gesture, representing the game rock-paper-scissors."
            }
            Icon::HandSparkles => "A hand with sparkles, representing magic or cleanliness.",
            Icon::HandSpock => "A hand making the Vulcan salute, representing Star Trek.",
            Icon::Handcuffs => "A pair of handcuffs, representing law enforcement or restraint.",
            Icon::Hands => "Two hands, indicating help or collaboration.",
            Icon::HandsAslInterpreting => "Hands signing in ASL, representing sign language.",
            Icon::HandsBound => {
                "A pair of hands bound together, representing restraint or solidarity."
            }
            Icon::HandsBubbles => "Hands with bubbles, representing washing or cleanliness.",
            Icon::HandsClapping => "Hands clapping, representing applause or appreciation.",
            Icon::HandsHolding => {
                "A pair of hands holding something, representing support or unity."
            }
            Icon::HandsHoldingChild => {
                "A pair of hands holding a child, representing care or protection."
            }
            Icon::HandsHoldingCircle => {
                "A pair of hands holding a circle, representing support or unity."
            }
            Icon::HandsPraying => "Hands in a praying position, representing prayer or hope.",
            Icon::Handshake => "Two hands shaking, indicating agreement or partnership.",
            Icon::HandshakeAngle => {
                "A handshake at an angle, representing agreement or partnership."
            }
            Icon::HandshakeSimple => "A simple handshake, representing agreement or partnership.",
            Icon::HandshakeSimpleSlash => "A handshake with a slash, indicating no agreement.",
            Icon::HandshakeSlash => {
                "Two hands shaking with a slash through them, indicating no agreement or social distancing."
            }
            Icon::Hanukiah => "A Hanukkah menorah, representing the Jewish festival.",
            Icon::HardDrive => "A hard drive, representing computer storage.",
            Icon::Hashtag => "A hashtag symbol, representing social media or categorization.",
            Icon::HatCowboy => "A cowboy hat, representing Western style.",
            Icon::HatCowboySide => "A cowboy hat from the side, representing Western style.",
            Icon::HatWizard => "A wizard hat, representing magic or fantasy.",
            Icon::HeadSideCough => "A head coughing, representing illness or discomfort.",
            Icon::HeadSideCoughSlash => "A head with a slash, indicating no coughing.",
            Icon::HeadSideMask => "A head with a mask, representing health protection.",
            Icon::HeadSideVirus => "A head with a virus, representing infection or illness.",
            Icon::Heading => "A large capital letter 'A', representing text heading.",
            Icon::Headphones => "A pair of headphones, indicating audio or music listening.",
            Icon::HeadphonesSimple => "Simple headphones, representing audio listening.",
            Icon::Headset => "A headset, representing audio communication.",
            Icon::Heart => "A heart shape, symbolizing love or likes.",
            Icon::HeartCircleBolt => "A heart with a circled bolt, representing energetic love.",
            Icon::HeartCircleCheck => {
                "A heart with a circled check mark, representing acceptance or love."
            }
            Icon::HeartCircleExclamation => {
                "A heart inside a circle with an exclamation mark, representing urgent health."
            }
            Icon::HeartCircleMinus => {
                "A heart inside a circle with a minus sign, representing health reduction."
            }
            Icon::HeartCirclePlus => {
                "A heart inside a circle with a plus sign, representing health or medical support."
            }
            Icon::HeartCircleXmark => "A heart with a circled `X`, representing rejection or loss.",
            Icon::HeartCrack => "A broken heart, representing heartbreak or sadness.",
            Icon::HeartPulse => "A heart with a pulse line, representing health or cardiology.",
            Icon::Helicopter => "A helicopter, representing aviation.",
            Icon::HelicopterSymbol => "A helicopter in flight, representing aviation.",
            Icon::HelmetSafety => "A safety helmet, representing protection or construction.",
            Icon::HelmetUn => "A helmet with 'UN', representing United Nations peacekeepers.",
            Icon::Highlighter => "A highlighter pen, representing marking or emphasis.",
            Icon::HillAvalanche => "A hill with an avalanche, representing snow slides.",
            Icon::HillRockslide => "A hill with a rockslide, representing landslides.",
            Icon::Hippo => "An icon of a hippo, representing the animal.",
            Icon::HockeyPuck => "A hockey puck, representing the sport of hockey.",
            Icon::HollyBerry => "A holly berry, representing Christmas or winter.",
            Icon::Horse => "A horse, representing the animal.",
            Icon::HorseHead => "A horse head, representing the animal or chess piece.",
            Icon::Hospital => "A hospital building, representing healthcare services.",
            Icon::HospitalUser => "A hospital with a user, representing a healthcare facility.",
            Icon::HotTubPerson => "A person in a hot tub, representing relaxation or spa.",
            Icon::Hotdog => "A hotdog, representing fast food.",
            Icon::Hotel => "A bed with a person, representing a hotel or accommodation.",
            Icon::Hourglass => "An hourglass, representing time.",
            Icon::HourglassEnd => "An hourglass almost empty, representing time running out.",
            Icon::HourglassHalf => "An hourglass half full, representing time.",
            Icon::HourglassStart => {
                "An hourglass with sand at the top, indicating the start of a timer."
            }
            Icon::House => "A simple outline of a house.",
            Icon::HouseChimney => "A house with a chimney, representing a home.",
            Icon::HouseChimneyCrack => "A house with a chimney and crack, representing damage.",
            Icon::HouseChimneyMedical => {
                "A house with a chimney and medical symbol, representing a home medical facility."
            }
            Icon::HouseChimneyUser => {
                "A house with a chimney and user, representing a home resident."
            }
            Icon::HouseChimneyWindow => "A house with a chimney and window, representing a home.",
            Icon::HouseCircleCheck => "A house with a circled check mark, representing approval.",
            Icon::HouseCircleExclamation => {
                "A house with a circled exclamation mark, representing caution."
            }
            Icon::HouseCircleXmark => "A house with a circled `X`, representing exclusion.",
            Icon::HouseCrack => "A house with a crack, representing damage or earthquake.",
            Icon::HouseFire => "A house with flames, representing a fire emergency.",
            Icon::HouseFlag => "A house with a flag, representing home pride.",
            Icon::HouseFloodWater => "A house with water, representing flooding.",
            Icon::HouseFloodWaterCircleArrowRight => {
                "A house with water and an arrow, representing flood direction."
            }
            Icon::HouseLaptop => "A house with a laptop, representing remote work or home office.",
            Icon::HouseLock => "A house with a lock, representing home security.",
            Icon::HouseMedical => "A house with a medical symbol, representing a medical facility.",
            Icon::HouseMedicalCircleCheck => {
                "A house with a medical symbol and a check mark, representing an approved medical facility."
            }
            Icon::HouseMedicalCircleExclamation => {
                "A house with a medical symbol and an exclamation mark, representing a medical facility with caution."
            }
            Icon::HouseMedicalCircleXmark => {
                "A house with a medical symbol and an `X`, representing a medical facility with restriction."
            }
            Icon::HouseMedicalFlag => {
                "A house with a medical flag, representing a medical facility."
            }
            Icon::HouseSignal => "A house with a signal, representing smart home or connectivity.",
            Icon::HouseTsunami => "A house with a tsunami wave, representing natural disaster.",
            Icon::HouseUser => "A house with a user inside, representing home or resident.",
            Icon::HryvniaSign => "The symbol for the Ukrainian hryvnia, indicating currency.",
            Icon::Hurricane => "A hurricane symbol, representing severe weather.",
            Icon::I => "The letter \"I\", representing the alphabet.",
            Icon::ICursor => "An I-beam cursor, representing text selection.",
            Icon::IceCream => "An ice cream cone, representing dessert or treats.",
            Icon::Icicles => "Ice formations, representing cold or winter.",
            Icon::Icons => "A collection of small icons, representing various symbols.",
            Icon::IdBadge => "An ID badge, representing identification.",
            Icon::IdCard => "An ID card, representing identification.",
            Icon::IdCardClip => "An ID card with a clip, representing identification.",
            Icon::Igloo => "An igloo, representing an Inuit house or cold regions.",
            Icon::Image => "A picture or photo icon, representing image content.",
            Icon::ImagePortrait => "A portrait image, representing photos or profiles.",
            Icon::Images => "Multiple images, representing photo galleries or collections.",
            Icon::Inbox => {
                "A tray filled with documents, representing an inbox or received messages."
            }
            Icon::Indent => "An indented line, representing text formatting.",
            Icon::IndianRupeeSign => "The symbol for the Indian rupee, indicating currency.",
            Icon::Industry => "A factory, representing industry or manufacturing.",
            Icon::Infinity => "An infinity symbol, representing limitless or infinite.",
            Icon::Info => "A lowercase 'i' in a circle, indicating information.",
            Icon::Instagram => "The logo of Instagram, representing the social media platform.",
            Icon::Italic => "An italic 'I', representing italicized text.",
            Icon::J => "A capital letter 'J', representing the letter.",
            Icon::Jar => "A simple jar, representing containers or storage.",
            Icon::JarWheat => "A jar with wheat, representing food storage.",
            Icon::Jedi => "The symbol for the Jedi Order, representing Star Wars or spirituality.",
            Icon::JediOrder => "The logo of Jedi Order, representing the Star Wars faction.",
            Icon::JetFighter => "A jet fighter, representing aviation or military.",
            Icon::JetFighterUp => "A jet fighter pointing up, representing aviation or military.",
            Icon::Joint => "Two linked circles, representing a connection or joint.",
            Icon::JugDetergent => "A jug of detergent, representing cleaning supplies.",
            Icon::K => "A capital letter 'K', representing the letter.",
            Icon::Kaaba => "The Kaaba, representing the holy site in Islam.",
            Icon::Key => "A key, representing access or security.",
            Icon::Keyboard => "A keyboard, representing typing or computing.",
            Icon::Khanda => "The symbol for Khanda, representing Sikhism.",
            Icon::Kickstarter => "The logo of Kickstarter, a crowdfunding platform.",
            Icon::KipSign => "The symbol for the Lao kip, indicating currency.",
            Icon::KitMedical => "A medical kit, representing health supplies.",
            Icon::KitchenSet => "Kitchen utensils, representing cooking or kitchen.",
            Icon::KiwiBird => "A kiwi bird, representing the bird or New Zealand.",
            Icon::L => "A capital letter 'L', representing the letter.",
            Icon::LandMineOn => "A land mine, representing explosive devices.",
            Icon::Landmark => "A landmark, representing important places or structures.",
            Icon::LandmarkDome => "A landmark with a dome, representing a notable building.",
            Icon::LandmarkFlag => "A landmark with a flag, representing a notable location.",
            Icon::Language => "A globe with characters, indicating language or translation.",
            Icon::Laptop => "A laptop computer, representing computing or work.",
            Icon::LaptopCode => "A laptop with code, representing programming or development.",
            Icon::LaptopFile => "A laptop with a file, representing digital documents.",
            Icon::LaptopMedical => {
                "A laptop with a medical symbol, representing telehealth or medical records."
            }
            Icon::LariSign => "The symbol for the Georgian lari, indicating currency.",
            Icon::LayerGroup => "Three stacked layers, indicating layering or grouping.",
            Icon::Leaf => "A leaf, representing nature or eco-friendliness.",
            Icon::LeftLong => "A long arrow pointing left, indicating direction.",
            Icon::LeftRight => {
                "An arrow pointing left and right, indicating bidirectional movement."
            }
            Icon::Lemon => "A lemon fruit, indicating the fruit or something sour.",
            Icon::LessThan => "A less than sign, representing mathematical operations.",
            Icon::LessThanEqual => {
                "A less than or equal sign, representing mathematical operations."
            }
            Icon::LifeRing => "A life ring, representing safety or rescue.",
            Icon::Lightbulb => "A lightbulb, representing ideas or illumination.",
            Icon::LinesLeaning => "Leaning lines, representing design or structure.",
            Icon::Link => "A chain link, indicating a hyperlink or connection.",
            Icon::LinkSlash => "A broken link, representing a disconnected hyperlink.",
            Icon::Linkedin => {
                "The logo of LinkedIn, representing the professional networking site."
            }
            Icon::LiraSign => "The symbol for the Italian lira, indicating currency.",
            Icon::List => "A simple list, representing items or data.",
            Icon::ListCheck => "A list with check marks, representing tasks or to-do lists.",
            Icon::ListOl => "An ordered list, representing a sequence or ranking.",
            Icon::ListUl => "A list with bullet points, representing unordered lists.",
            Icon::LitecoinSign => "The symbol for Litecoin, indicating cryptocurrency.",
            Icon::LocationArrow => "An arrow on a map, representing direction.",
            Icon::LocationCrosshairs => "A crosshairs on a map, representing target location.",
            Icon::LocationDot => "A pinpoint marker, indicating a location on a map.",
            Icon::LocationPin => "A pin marker, indicating a specific location.",
            Icon::LocationPinLock => "A location pin with a lock, representing secure location.",
            Icon::Lock => "A padlock, representing security or privacy.",
            Icon::LockOpen => "A padlock that is open, representing access or security.",
            Icon::Locust => "A locust, representing the insect or a plague.",
            Icon::Lungs => "A pair of lungs, representing the respiratory system.",
            Icon::LungsVirus => "A pair of lungs with a virus, representing respiratory illness.",
            Icon::M => "A capital letter 'M', representing the letter.",
            Icon::Magnet => "A horseshoe magnet, representing attraction or magnetic fields.",
            Icon::MagnifyingGlass => {
                "A magnifying glass, often used to represent search functionality."
            }
            Icon::MagnifyingGlassArrowRight => {
                "A magnifying glass with a right arrow, representing search direction."
            }
            Icon::MagnifyingGlassChart => {
                "A magnifying glass with a chart, representing detailed analysis."
            }
            Icon::MagnifyingGlassDollar => {
                "A magnifying glass with a dollar sign, representing financial search."
            }
            Icon::MagnifyingGlassLocation => {
                "A magnifying glass over a location pin, representing search location."
            }
            Icon::MagnifyingGlassMinus => {
                "A magnifying glass with a minus sign, representing zoom out or search."
            }
            Icon::MagnifyingGlassPlus => {
                "A magnifying glass with a plus sign, representing zoom in or search."
            }
            Icon::ManatSign => "The symbol for the Azerbaijani manat, indicating currency.",
            Icon::Map => "A folded map, representing navigation or geography.",
            Icon::MapLocation => "A map with a pin, representing location or navigation.",
            Icon::MapLocationDot => "A map pin with a dot, representing location or navigation.",
            Icon::MapPin => "A map pin, representing location or navigation.",
            Icon::Marker => "A marker, representing writing or drawing tools.",
            Icon::Mars => "The symbol for Mars, representing the planet or male gender.",
            Icon::MarsAndVenus => {
                "The symbols for Mars and Venus, representing gender or relationships."
            }
            Icon::MarsAndVenusBurst => {
                "The symbols for Mars and Venus with a burst, indicating gender diversity."
            }
            Icon::MarsDouble => "Two Mars symbols, representing male gender or masculinity.",
            Icon::MarsStroke => {
                "The Mars stroke symbol, representing a variation of the male gender symbol."
            }
            Icon::MarsStrokeRight => {
                "The symbol for Mars with a right arrow, indicating male gender or masculinity."
            }
            Icon::MarsStrokeUp => {
                "The symbol for Mars with an upward arrow, indicating male gender or masculinity."
            }
            Icon::MartiniGlass => "A martini glass, representing beverages or cocktails.",
            Icon::MartiniGlassCitrus => {
                "A martini glass with a citrus slice, representing beverages or cocktails."
            }
            Icon::MartiniGlassEmpty => {
                "An empty martini glass, representing beverages or cocktails."
            }
            Icon::Mask => "A theater mask, representing performance or disguise.",
            Icon::MaskFace => "A face mask, representing health or safety.",
            Icon::MaskVentilator => "A medical mask, representing health protection.",
            Icon::MasksTheater => "Two theater masks, representing performance or drama.",
            Icon::MattressPillow => "A mattress with a pillow, representing bedding or sleep.",
            Icon::Maximize => "A square with arrows pointing outwards, indicating maximization.",
            Icon::Medal => "A medal, representing achievement or award.",
            Icon::Medium => "The logo of Medium, a publishing platform.",
            Icon::Memory => "A microchip, representing memory or computing hardware.",
            Icon::Menorah => "A menorah, representing the Jewish candelabrum.",
            Icon::Mercury => {
                "The symbol for the planet Mercury, representing the celestial body or the element."
            }
            Icon::Message => "A speech bubble, representing communication or messaging.",
            Icon::Meteor => "A meteor, representing space or celestial events.",
            Icon::Microchip => "A microchip, representing technology or computing.",
            Icon::Microphone => "A microphone, representing audio or recording.",
            Icon::MicrophoneLines => {
                "A microphone with lines, representing audio recording or broadcasting."
            }
            Icon::MicrophoneLinesSlash => {
                "A microphone with a slash, indicating no audio recording."
            }
            Icon::MicrophoneSlash => "A microphone with a slash, indicating mute or no sound.",
            Icon::Microscope => "A microscope, representing science or research.",
            Icon::MillSign => "A sign for mills, representing currency or measurement.",
            Icon::Minimize => "A minimized window, representing reduction.",
            Icon::Minus => "A minus sign, indicating subtraction or decrease.",
            Icon::Mitten => "A mitten, representing winter clothing.",
            Icon::Mobile => "A mobile phone, indicating communication or devices.",
            Icon::MobileButton => {
                "A mobile phone with buttons, representing old-style mobile device."
            }
            Icon::MobileRetro => "A retro mobile phone, representing old technology.",
            Icon::MobileScreen => "A mobile phone, representing mobile device.",
            Icon::MobileScreenButton => "A mobile phone with a button, representing mobile device.",
            Icon::MoneyBill => "A paper bill, representing money or currency.",
            Icon::MoneyBill1 => "A money bill, representing payment or currency.",
            Icon::MoneyBill1Wave => {
                "A money bill with a wave, representing payment or transaction."
            }
            Icon::MoneyBillTransfer => "A money bill with an arrow, indicating financial transfer.",
            Icon::MoneyBillTrendUp => {
                "A money bill with an upward trend, representing financial growth."
            }
            Icon::MoneyBillWave => "A waving money bill, representing cash flow.",
            Icon::MoneyBillWheat => {
                "A money bill with wheat, representing agricultural subsidy or trade."
            }
            Icon::MoneyBills => "A stack of money bills, representing wealth or currency.",
            Icon::MoneyCheck => "A check, representing financial transactions.",
            Icon::MoneyCheckDollar => {
                "A check with a dollar sign, representing financial transactions."
            }
            Icon::Monument => "A monument, representing historical or cultural significance.",
            Icon::Moon => "A crescent moon, representing night or sleep mode.",
            Icon::MortarPestle => "A mortar and pestle, representing grinding or pharmacy.",
            Icon::Mosque => "A mosque, representing Islamic place of worship.",
            Icon::Mosquito => "A mosquito, representing the insect or disease vector.",
            Icon::MosquitoNet => "A mosquito net, representing protection from insects.",
            Icon::Motorcycle => "A motorcycle, representing motorbiking.",
            Icon::Mound => "A mound of earth, representing a small hill or pile.",
            Icon::Mountain => "A mountain, representing nature or hiking.",
            Icon::MountainCity => {
                "A cityscape with mountains, representing urban and natural landscape."
            }
            Icon::MountainSun => "A mountain with a sun, indicating landscape or outdoors.",
            Icon::MugHot => "A hot mug, representing a hot beverage.",
            Icon::MugSaucer => "A mug on a saucer, representing coffee or tea.",
            Icon::Music => "A musical note, representing music or audio.",
            Icon::N => "A capital letter 'N', representing the letter.",
            Icon::NairaSign => "The symbol for the Nigerian naira, indicating currency.",
            Icon::Napster => "The logo of Napster, representing the music streaming service.",
            Icon::NetworkWired => "A network of connected nodes, representing wired networking.",
            Icon::Neuter => "The gender symbol for neuter, indicating neutrality.",
            Icon::Newspaper => "A newspaper, indicating news or publications.",
            Icon::NfcDirectional => {
                "The logo of NFC Directional, representing near-field communication."
            }
            Icon::NfcSymbol => {
                "The NFC (Near Field Communication) symbol, representing wireless communication."
            }
            Icon::NotEqual => "A not equal sign, indicating inequality or difference.",
            Icon::Notdef => "The .notdef glyph, representing missing characters in typography.",
            Icon::NoteSticky => "A sticky note, representing reminders or notes.",
            Icon::NotesMedical => {
                "A clipboard with medical notes, representing healthcare documentation."
            }
            Icon::O => "A capital letter 'O', representing the letter or shape.",
            Icon::ObjectGroup => "An icon of grouped objects, indicating grouping.",
            Icon::ObjectUngroup => "An icon of separated objects, indicating ungrouping.",
            Icon::OilCan => "An oil can, representing lubrication or mechanics.",
            Icon::OilWell => "An oil well, representing fossil fuels or drilling.",
            Icon::OldRepublic => "The logo of Old Republic, representing the Star Wars faction.",
            Icon::Om => "The Om symbol, representing Hinduism.",
            Icon::Otter => "An otter, representing the animal.",
            Icon::Outdent => "Text with a reduced indent, representing text alignment.",
            Icon::P => "A capital letter 'P', representing the letter or parking.",
            Icon::Pager => "A pager, representing communication devices.",
            Icon::PaintRoller => "A paint roller, indicating painting or renovation.",
            Icon::Paintbrush => "A paintbrush, representing painting or art.",
            Icon::Palette => "A painter's palette, representing art or color selection.",
            Icon::Pallet => "A pallet, representing shipping or logistics.",
            Icon::Panorama => "A wide-angle view, representing landscape photography.",
            Icon::PaperPlane => "A paper plane, indicating sending a message or flying.",
            Icon::Paperclip => "A paperclip, representing attachment or link.",
            Icon::ParachuteBox => "A box with a parachute, representing delivery or drop.",
            Icon::Paragraph => "A paragraph symbol, representing text.",
            Icon::Passport => "A passport, representing international travel.",
            Icon::Paste => "A clipboard with a document, representing pasting.",
            Icon::Pause => "A pause symbol, representing media pause.",
            Icon::Paw => "A paw print, representing animals or pets.",
            Icon::Paypal => "The logo of PayPal, an online payment system.",
            Icon::Peace => "A peace symbol, representing peace or anti-war.",
            Icon::Pen => "A pen, representing writing or creativity.",
            Icon::PenClip => "A pen with a clip, representing writing or stationery.",
            Icon::PenFancy => "A fancy pen, representing writing or creativity.",
            Icon::PenNib => "An old-fashioned pen nib, representing writing or creativity.",
            Icon::PenRuler => "A pen and ruler, representing drawing or design.",
            Icon::PenToSquare => "A pen writing on a square, indicating editing or writing.",
            Icon::Pencil => "A pencil, representing writing or editing.",
            Icon::PeopleArrows => {
                "Two people with arrows pointing towards each other, representing communication or interaction."
            }
            Icon::PeopleCarryBox => "People carrying a box, representing moving or teamwork.",
            Icon::PeopleGroup => "Multiple people, representing a group or community.",
            Icon::PeopleLine => "People standing in line, representing queue.",
            Icon::PeoplePulling => "Two people pulling, representing teamwork or effort.",
            Icon::PeopleRobbery => "A person being robbed, representing crime or danger.",
            Icon::PeopleRoof => "People under a roof, representing shelter or protection.",
            Icon::PepperHot => "A hot pepper, representing spicy food.",
            Icon::Percent => "A percent sign, indicating percentages or discounts.",
            Icon::Person => "A person, representing an individual or user.",
            Icon::PersonArrowDownToLine => {
                "A person with an arrow pointing down to a line, indicating descending or moving down."
            }
            Icon::PersonArrowUpFromLine => {
                "A person with an arrow pointing up from a line, indicating rising or moving up."
            }
            Icon::PersonBiking => "A person biking, representing cycling.",
            Icon::PersonBooth => "A person in a booth, indicating privacy or voting.",
            Icon::PersonBreastfeeding => {
                "A person breastfeeding, representing motherhood or childcare."
            }
            Icon::PersonBurst => "A person with a burst, indicating excitement or energy.",
            Icon::PersonCane => "A person with a cane, representing disability or assistance.",
            Icon::PersonChalkboard => {
                "A person at a chalkboard, representing teaching or presentation."
            }
            Icon::PersonCircleCheck => {
                "A person inside a circle with a check mark, representing verification."
            }
            Icon::PersonCircleExclamation => {
                "A person with a circled exclamation mark, indicating warning."
            }
            Icon::PersonCircleMinus => {
                "A person with a circled minus, indicating removal or exclusion."
            }
            Icon::PersonCirclePlus => {
                "A person with a circled plus, indicating addition or inclusion."
            }
            Icon::PersonCircleQuestion => {
                "A person with a circled question mark, indicating inquiry or uncertainty."
            }
            Icon::PersonCircleXmark => "A person with a circled `X`, indicating exclusion.",
            Icon::PersonDigging => "A person digging, representing construction or excavation.",
            Icon::PersonDotsFromLine => {
                "A person with dots moving from a line, representing transition or movement."
            }
            Icon::PersonDress => "A person wearing a dress, representing clothing or fashion.",
            Icon::PersonDressBurst => {
                "A person in a dress with a burst, indicating excitement or motion."
            }
            Icon::PersonDrowning => "A person drowning, representing danger in water.",
            Icon::PersonFalling => "A person falling, representing accident or failure.",
            Icon::PersonFallingBurst => {
                "A person falling with a burst, representing injury or accident."
            }
            Icon::PersonHalfDress => {
                "A person wearing half a dress, representing fashion or gender fluidity."
            }
            Icon::PersonHarassing => "A person harassing another, representing harassment.",
            Icon::PersonHiking => "A person hiking, representing outdoor activities.",
            Icon::PersonMilitaryPointing => {
                "A military person pointing, indicating direction or command."
            }
            Icon::PersonMilitaryRifle => {
                "A military person holding a rifle, representing armed forces."
            }
            Icon::PersonMilitaryToPerson => {
                "A military person saluting another person, representing respect."
            }
            Icon::PersonPraying => "A person praying, representing spirituality or religion.",
            Icon::PersonPregnant => "A pregnant person, representing pregnancy.",
            Icon::PersonRays => "A person with rays, representing radiance or positivity.",
            Icon::PersonRifle => {
                "A person holding a rifle, representing shooting sports or military."
            }
            Icon::PersonRunning => "A person running, representing movement or exercise.",
            Icon::PersonShelter => "A person under a shelter, representing protection or safety.",
            Icon::PersonSkating => "A person skating, representing the sport or activity.",
            Icon::PersonSkiing => "A person skiing, representing winter sports.",
            Icon::PersonSkiingNordic => "A person skiing Nordic style, representing skiing.",
            Icon::PersonSnowboarding => "A person snowboarding, representing winter sports.",
            Icon::PersonSwimming => "A person swimming, representing swimming or water sports.",
            Icon::PersonThroughWindow => {
                "A person moving through a window, indicating escape or emergency exit."
            }
            Icon::PersonWalking => "A person walking, representing movement.",
            Icon::PersonWalkingArrowLoopLeft => {
                "A person walking with a looping arrow to the left, indicating return or reverse."
            }
            Icon::PersonWalkingArrowRight => "A person walking with an arrow, indicating movement.",
            Icon::PersonWalkingDashedLineArrowRight => {
                "A person walking with a dashed line and arrow, indicating a guided path."
            }
            Icon::PersonWalkingLuggage => "A person walking with luggage, indicating travel.",
            Icon::PersonWalkingWithCane => {
                "A person walking with a cane, indicating disability or assistance."
            }
            Icon::PesetaSign => "The symbol for the Spanish peseta, indicating currency.",
            Icon::PesoSign => "The symbol for the Philippine peso, indicating currency.",
            Icon::Phone => "A phone, representing communication or contact.",
            Icon::PhoneFlip => "A phone flipped, indicating mobile communication.",
            Icon::PhoneSlash => "A phone with a slash, indicating no calls.",
            Icon::PhoneVolume => {
                "A phone handset with sound waves, indicating a call or audio settings."
            }
            Icon::PhotoFilm => "A strip of photo film, representing photography.",
            Icon::PiggyBank => "A piggy bank, representing savings or finance.",
            Icon::Pills => "A pair of pills, representing medication.",
            Icon::PizzaSlice => "A slice of pizza, representing food or dining.",
            Icon::PlaceOfWorship => "A place of worship, indicating religious services.",
            Icon::Plane => "An airplane, indicating travel or flights.",
            Icon::PlaneArrival => "A plane arriving, indicating air travel arrival.",
            Icon::PlaneCircleCheck => {
                "A plane with a circled check mark, representing flight confirmation."
            }
            Icon::PlaneCircleExclamation => {
                "A plane with a circled exclamation mark, indicating travel alert."
            }
            Icon::PlaneCircleXmark => "A plane with a circled `X`, indicating no flying.",
            Icon::PlaneDeparture => "A plane taking off, indicating air travel.",
            Icon::PlaneLock => "A plane with a lock, indicating secure travel.",
            Icon::PlaneSlash => "A plane with a slash, indicating no flying.",
            Icon::PlaneUp => "A plane with the tip upwards.",
            Icon::PlantWilt => "A wilted plant, indicating lack of water or poor health.",
            Icon::PlateWheat => "A plate with wheat, indicating food or meal.",
            Icon::Play => "A play button, indicating media playback.",
            Icon::Playstation => "The logo of PlayStation, a gaming console.",
            Icon::Plug => "An electrical plug, indicating power or connectivity.",
            Icon::PlugCircleBolt => "A plug with a circled bolt, indicating powered connection.",
            Icon::PlugCircleCheck => "A plug with a circled check, indicating secure connection.",
            Icon::PlugCircleExclamation => {
                "A plug with a circled exclamation mark, representing power alert."
            }
            Icon::PlugCircleMinus => {
                "A plug with a circled minus sign, representing power reduction."
            }
            Icon::PlugCirclePlus => "A plug with a circled plus, indicating connection.",
            Icon::PlugCircleXmark => "A plug with a circled `X`, indicating no connection.",
            Icon::Plus => "A cross, representing addition or positivity.",
            Icon::PlusMinus => "A plus and minus sign, indicating addition and subtraction.",
            Icon::Podcast => "A podcast icon, representing audio broadcasting.",
            Icon::Poo => "A pile of poo with eyes, often used humorously.",
            Icon::PooStorm => "A storm cloud with a poo, often used humorously.",
            Icon::Poop => "A pile of poop, representing waste or humor.",
            Icon::PowerOff => "A power button, indicating shutdown or turning off.",
            Icon::Prescription => "A prescription symbol, indicating medical prescription.",
            Icon::PrescriptionBottle => {
                "A prescription bottle, representing medicine or healthcare."
            }
            Icon::PrescriptionBottleMedical => {
                "A medical prescription bottle, indicating medication."
            }
            Icon::Print => "A printer, representing printing documents.",
            Icon::PumpMedical => "A medical pump, indicating medical equipment.",
            Icon::PumpSoap => "A soap dispenser, representing hygiene or cleanliness.",
            Icon::PuzzlePiece => "A puzzle piece, indicating a part of a puzzle.",
            Icon::Q => "The letter \"Q\", representing the alphabet.",
            Icon::Qrcode => "A QR code, representing quick response codes for scanning.",
            Icon::Question => "A question mark, indicating inquiry or help.",
            Icon::QuoteLeft => "A left-leaning quotation mark, indicating the start of a quote.",
            Icon::QuoteRight => "A right-leaning quotation mark, indicating the end of a quote.",
            Icon::R => "A capital letter 'R', representing the letter or registered trademark.",
            Icon::Radiation => "A radiation symbol, indicating hazardous materials.",
            Icon::Radio => "A radio, representing broadcasting or communication.",
            Icon::Rainbow => "A rainbow, representing LGBTQ+ pride or spectrum.",
            Icon::RankingStar => "A star with a number, indicating rank or rating.",
            Icon::Receipt => "A receipt, representing a transaction record.",
            Icon::RecordVinyl => "A vinyl record, representing music or audio.",
            Icon::RectangleAd => "A rectangle with 'AD' inside, indicating advertisement.",
            Icon::RectangleList => "A rectangle with a list inside, representing menu or options.",
            Icon::RectangleXmark => "A rectangle with an `X`, indicating deletion or closure.",
            Icon::Recycle => "Three arrows forming a triangle, indicating recycling.",
            Icon::Registered => "A circled 'R', indicating a registered trademark.",
            Icon::Repeat => "Two arrows forming a circle, indicating repeat or refresh.",
            Icon::Reply => "An arrow pointing left, indicating a reply.",
            Icon::ReplyAll => "A reply-all symbol, representing email or messaging.",
            Icon::Republican => "An elephant, representing the Republican party.",
            Icon::Restroom => "A man and woman icon, indicating restroom facilities.",
            Icon::Retweet => "Two arrows forming a square, indicating retweet or repost.",
            Icon::Ribbon => "A ribbon, representing awareness or decoration.",
            Icon::RightFromBracket => {
                "An arrow pointing right from a bracket, indicating exit or move."
            }
            Icon::RightLeft => {
                "An arrow pointing right and left, indicating bidirectional movement."
            }
            Icon::RightLong => "A long arrow pointing right, indicating forward direction.",
            Icon::RightToBracket => {
                "An arrow pointing right into a bracket, indicating entering or logging in."
            }
            Icon::Ring => "A ring, representing jewelry or engagement.",
            Icon::Road => "A road, indicating travel or transportation.",
            Icon::RoadBarrier => "A road barrier, indicating roadblock or construction.",
            Icon::RoadBridge => "A bridge, representing transportation infrastructure.",
            Icon::RoadCircleCheck => {
                "A road with a circled check mark, representing approved routes."
            }
            Icon::RoadCircleExclamation => {
                "A road with a circled exclamation mark, indicating caution or warning."
            }
            Icon::RoadCircleXmark => "A road with a circled `X`, indicating road closure.",
            Icon::RoadLock => "A road with a lock, indicating restricted access.",
            Icon::RoadSpikes => "Spikes on the road, representing security or vehicle stop.",
            Icon::Robot => "A robot, representing automation or robotics.",
            Icon::Rocket => "A rocket, indicating space exploration or rapid progress.",
            Icon::Rotate => "A circular arrow, indicating rotation or refresh.",
            Icon::RotateLeft => "An arrow rotating to the left, indicating undo or backward.",
            Icon::RotateRight => "An arrow rotating to the right, indicating redo or refresh.",
            Icon::Route => "A winding road, indicating a path or journey.",
            Icon::Rss => "A feed icon, representing RSS feed.",
            Icon::RubleSign => "The symbol for the Russian ruble, indicating currency.",
            Icon::Rug => "A rug, representing home decor or carpeting.",
            Icon::Ruler => "A ruler, representing measurement.",
            Icon::RulerCombined => "A ruler combined with another tool, representing measurement.",
            Icon::RulerHorizontal => "A horizontal ruler, representing measurement.",
            Icon::RulerVertical => "A vertical ruler, representing measurement.",
            Icon::RupeeSign => "The symbol for the Indian rupee, indicating currency in letters",
            Icon::RupiahSign => "The symbol for the Indonesian rupiah, indicating currency.",
            Icon::S => "A capital letter 'S', representing the letter or Superman.",
            Icon::SackDollar => "A sack with a dollar sign, indicating money or wealth.",
            Icon::SackXmark => "A sack with an `X`, indicating no contents or emptiness.",
            Icon::Sailboat => "A sailboat, representing sailing or maritime activities.",
            Icon::Satellite => "A satellite, representing space or communication.",
            Icon::SatelliteDish => "A satellite dish, representing communication.",
            Icon::ScaleBalanced => "A balanced scale, representing justice or equality.",
            Icon::ScaleUnbalanced => "A tilted scale, indicating imbalance.",
            Icon::ScaleUnbalancedFlip => {
                "A tilted scale, indicating imbalance, flipped horizzontally."
            }
            Icon::School => "A school building, representing education.",
            Icon::SchoolCircleCheck => {
                "A school building with a check mark in a circle, indicating school approval."
            }
            Icon::SchoolCircleExclamation => {
                "A school with a circled exclamation mark, representing school alert."
            }
            Icon::SchoolCircleXmark => {
                "A school with a circled X, representing school closure or cancellation."
            }
            Icon::SchoolFlag => {
                "A school building with a flag, representing education or school pride."
            }
            Icon::SchoolLock => "A school building with a lock, indicating school security.",
            Icon::Scissors => "A pair of scissors, representing cutting or crafting.",
            Icon::Screwdriver => "A screwdriver, representing tools or repair.",
            Icon::ScrewdriverWrench => {
                "A screwdriver and wrench crossed, representing tools or repair."
            }
            Icon::Scroll => "A scroll, representing a document or parchment.",
            Icon::ScrollTorah => "A scroll, representing the Torah or ancient texts.",
            Icon::SdCard => "An SD card, representing storage or memory.",
            Icon::Section => "A divided section, representing a part or segment.",
            Icon::Seedling => "A small plant sprouting, representing growth or new beginnings.",
            Icon::Server => "A server, representing data storage or hosting.",
            Icon::Shapes => "A collection of geometric shapes, representing design or layout.",
            Icon::Share => "An arrow pointing outwards, indicating sharing content.",
            Icon::ShareFromSquare => {
                "An arrow coming out of a square, indicating sharing or exporting."
            }
            Icon::ShareNodes => "Three connected nodes, representing sharing or networking.",
            Icon::SheetPlastic => "A sheet of plastic, representing material.",
            Icon::ShekelSign => "The symbol for the Israeli shekel, indicating currency.",
            Icon::Shield => "A shield, representing protection or security.",
            Icon::ShieldCat => "A shield with a cat, representing pet protection.",
            Icon::ShieldDog => "A shield with a dog, representing pet protection.",
            Icon::ShieldHalved => {
                "A shield split in half, indicating partial protection or security."
            }
            Icon::ShieldHeart => "A shield with a heart, representing health protection.",
            Icon::ShieldVirus => "A shield with a virus, representing antivirus protection.",
            Icon::Ship => "A ship, representing maritime transportation.",
            Icon::Shirt => "A t-shirt, indicating clothing.",
            Icon::ShoePrints => "Shoe prints, representing footsteps or tracking.",
            Icon::Shop => "A store front, indicating shopping or retail.",
            Icon::ShopLock => "A shop with a lock, representing a closed store.",
            Icon::ShopSlash => "A shop with a slash, indicating closed or no shopping.",
            Icon::Shopify => "The logo of Shopify, an e-commerce platform.",
            Icon::Shower => "A shower head with water, indicating bathing.",
            Icon::Shrimp => "A shrimp, representing seafood.",
            Icon::Shuffle => "Two arrows crossing, indicating shuffle or random order.",
            Icon::ShuttleSpace => "A space shuttle, representing space exploration.",
            Icon::SignHanging => "A hanging sign, representing a signboard or notice.",
            Icon::Signal => {
                "A signal tower with waves, representing communication or connectivity."
            }
            Icon::Signature => "A handwritten signature, indicating signing or approval.",
            Icon::SignsPost => "A signpost, representing directions or navigation.",
            Icon::SimCard => "A SIM card, representing mobile connectivity.",
            Icon::Sink => "A sink, representing kitchen or bathroom fixtures.",
            Icon::Sitemap => {
                "A hierarchical diagram, representing a sitemap or organization chart."
            }
            Icon::Skull => "A simple skull, representing death or danger.",
            Icon::SkullCrossbones => "A skull with crossbones, representing danger or pirates.",
            Icon::Slack => "The logo of Slack, a communication platform for teams.",
            Icon::Slash => "A slash symbol, representing separation or division.",
            Icon::Sleigh => "A sleigh, representing Christmas or winter transport.",
            Icon::Sliders => "Sliders, representing controls or adjustments.",
            Icon::Smog => "A city skyline with smog, representing air pollution.",
            Icon::Smoking => "A cigarette with smoke, representing smoking.",
            Icon::Snowflake => "A snowflake, representing cold or winter.",
            Icon::Snowman => "A snowman, representing winter or Christmas.",
            Icon::Snowplow => "A snowplow vehicle, representing snow removal.",
            Icon::Soap => "A bar of soap, representing cleanliness or hygiene.",
            Icon::Socks => "A pair of socks, representing clothing.",
            Icon::SolarPanel => "A solar panel, representing solar energy.",
            Icon::Sort => "Three stacked horizontal lines, indicating sorting.",
            Icon::SortDown => {
                "A list with a downward arrow, indicating sorting in descending order."
            }
            Icon::SortUp => "A list with an upward arrow, indicating sorting in ascending order.",
            Icon::Soundcloud => "The logo of SoundCloud, representing the music platform.",
            Icon::Spa => "A flower with petals, representing relaxation or spa.",
            Icon::SpaceAwesome => "The logo of Space Awesome, representing the brand or company.",
            Icon::SpaghettiMonsterFlying => {
                "A flying spaghetti monster, representing parody religion."
            }
            Icon::SpellCheck => "A check mark with ABC, representing spell checking.",
            Icon::Spider => "A spider, representing the arachnid or Halloween.",
            Icon::Spinner => "A spinning circle, indicating loading or processing.",
            Icon::Splotch => "A paint splotch, representing color or mess.",
            Icon::Spoon => "A spoon, representing dining or kitchen utensils.",
            Icon::Spotify => "The logo of Spotify, a music streaming service.",
            Icon::SprayCan => "A spray can, representing painting or spraying.",
            Icon::SprayCanSparkles => "A spray can emitting sparkles, representing spray effects.",
            Icon::Square => "A simple square, representing shape or stop.",
            Icon::SquareArrowUpRight => {
                "A square with an arrow pointing up and right, indicating expansion or exit."
            }
            Icon::SquareCaretDown => {
                "A square with a downward caret, representing more options or dropdowns."
            }
            Icon::SquareCaretLeft => {
                "A square with a leftward caret, representing navigation or more options."
            }
            Icon::SquareCaretRight => {
                "A square with a rightward caret, representing navigation or more options."
            }
            Icon::SquareCaretUp => {
                "A square with an upward caret, representing navigation or more options."
            }
            Icon::SquareCheck => "A square with a check mark, indicating completion or approval.",
            Icon::SquareEnvelope => "A square with an envelope, representing mail or messages.",
            Icon::SquareFull => {
                "A square completely filled, representing fullness or completeness."
            }
            Icon::SquareH => "A square with an 'H', representing hospital.",
            Icon::SquareMinus => "A square with a minus sign, indicating removal or decrease.",
            Icon::SquareNfi => "A square with 'NFI', indicating an undefined acronym.",
            Icon::SquareParking => "A square with a 'P', representing parking.",
            Icon::SquarePen => "A square with a pen, representing editing or writing.",
            Icon::SquarePersonConfined => {
                "A square with a person confined inside, representing isolation."
            }
            Icon::SquarePhone => {
                "A square with a phone icon, representing communication or device."
            }
            Icon::SquarePhoneFlip => {
                "A square with a phone icon flipped, indicating phone rotation."
            }
            Icon::SquarePlus => "A square with a plus sign, indicating addition or increase.",
            Icon::SquarePollHorizontal => {
                "A square with horizontal bars, representing a horizontal poll or chart."
            }
            Icon::SquarePollVertical => {
                "A square with vertical bars, representing a vertical poll or chart."
            }
            Icon::SquareRootVariable => {
                "A square with a variable inside a root symbol, representing mathematics."
            }
            Icon::SquareRss => "A square with RSS icon, representing news feed.",
            Icon::SquareShareNodes => {
                "A square with nodes connected by lines, indicating sharing or networking."
            }
            Icon::SquareSteam => "A square with the Steam logo, representing the gaming platform.",
            Icon::SquareUpRight => {
                "A square with a bold arrow pointing up and right, indicating expansion or exit."
            }
            Icon::SquareVirus => "A square with virus icons, representing illness or infection.",
            Icon::SquareXmark => "A square with an `X`, representing rejection or closure.",
            Icon::Squarespace => "The logo of Squarespace, a website building platform.",
            Icon::StackOverflow => "The logo of Stack Overflow, a Q&A platform for developers.",
            Icon::StaffSnake => "A staff with a snake, representing medical profession.",
            Icon::Stairs => "A staircase, representing steps or levels.",
            Icon::Stamp => "A stamp, representing approval or postage.",
            Icon::Stapler => "A stapler, representing office supplies.",
            Icon::Star => "A star, often used to represent favorites or ratings.",
            Icon::StarAndCrescent => "A star and crescent, representing Islam.",
            Icon::StarHalf => "A half-filled star, indicating partial rating.",
            Icon::StarHalfStroke => {
                "A half-filled star, indicating partial rating, with a stroke around it."
            }
            Icon::StarOfDavid => "A star of David, representing Judaism.",
            Icon::StarOfLife => {
                "A six-pointed star with a rod in the center, representing emergency medical services."
            }
            Icon::Steam => "The logo of Steam, representing the gaming platform.",
            Icon::SteamSymbol => {
                "The logo of Steam, representing the gaming platform, with white background."
            }
            Icon::SterlingSign => "The symbol for the British pound, indicating currency.",
            Icon::Stethoscope => "A stethoscope, representing medical examination or healthcare.",
            Icon::Stop => "A stop sign, indicating cessation or pause.",
            Icon::Stopwatch => "A simple stopwatch, representing timing.",
            Icon::Stopwatch20 => "A stopwatch showing 20 seconds, representing time measurement.",
            Icon::Store => "A storefront, representing retail or shops.",
            Icon::StoreSlash => "A store with a slash, indicating closed or no store.",
            Icon::StreetView => "A street view symbol, representing navigation or mapping.",
            Icon::Strikethrough => {
                "Text with a line through it, indicating deletion or correction."
            }
            Icon::Stripe => "The logo of Stripe, a payment processing platform.",
            Icon::StripeS => "The logo of Stripe, representing the payment processing platform.",
            Icon::Stroopwafel => "A stroopwafel, representing the Dutch treat.",
            Icon::Subscript => "A subscript 'A', indicating subscript text.",
            Icon::Suitcase => "A simple suitcase, representing travel or luggage.",
            Icon::SuitcaseMedical => "A medical suitcase, representing emergency medical kit.",
            Icon::SuitcaseRolling => "A suitcase with wheels, representing travel.",
            Icon::Sun => "A sun, representing daytime or brightness.",
            Icon::SunPlantWilt => "A sun with a wilted plant, indicating drought or plant stress.",
            Icon::Superscript => "A superscript 'A', indicating superscript text.",
            Icon::Swatchbook => "A swatchbook, representing color samples or design.",
            Icon::Synagogue => "A synagogue, representing a place of worship for Jews.",
            Icon::Syringe => "A syringe, representing medical injections.",
            Icon::T => "A capital letter 'T', representing the letter.",
            Icon::Table => "A simple table, indicating data or spreadsheet.",
            Icon::TableCells => "A table with cells, representing data organization.",
            Icon::TableCellsColumnLock => "A table with a locked column, indicating fixed data.",
            Icon::TableCellsLarge => "A table with large cells, representing data organization.",
            Icon::TableCellsRowLock => "A table with a locked row, indicating fixed data.",
            Icon::TableColumns => "A table with columns, representing data organization.",
            Icon::TableList => "A table with a list, representing data organization.",
            Icon::TableTennisPaddleBall => {
                "A table tennis paddle with a ball, representing the sport."
            }
            Icon::Tablet => "A tablet device, representing mobile computing.",
            Icon::TabletButton => "A tablet with a button, representing a touchscreen device.",
            Icon::TabletScreenButton => {
                "A tablet with a screen and button, representing a digital device."
            }
            Icon::Tablets => "Two pills, representing medication or tablets.",
            Icon::TachographDigital => "A digital tachograph, representing vehicle monitoring.",
            Icon::Tag => "A price tag, indicating labels or pricing.",
            Icon::Tags => "Multiple tags, representing labels or categories.",
            Icon::Tape => "A roll of tape, representing adhesive tape.",
            Icon::Tarp => "A simple tarp, representing a cover or protection.",
            Icon::TarpDroplet => "A tarp with a droplet, representing waterproof covering.",
            Icon::Taxi => "A taxi cab, representing transportation service.",
            Icon::Teeth => "A set of teeth, representing dental health.",
            Icon::TeethOpen => "An open mouth with teeth, representing dental health or smiling.",
            Icon::TemperatureArrowDown => {
                "A thermometer with a downward arrow, indicating falling temperature."
            }
            Icon::TemperatureArrowUp => {
                "A thermometer with an upward arrow, indicating rising temperature."
            }
            Icon::TemperatureEmpty => "A thermometer empty, representing no temperature reading.",
            Icon::TemperatureFull => "A thermometer full, representing very high temperature.",
            Icon::TemperatureHalf => "A thermometer half full, representing moderate temperature.",
            Icon::TemperatureHigh => {
                "A thermometer with high reading, indicating high temperature."
            }
            Icon::TemperatureLow => "A thermometer with low reading, indicating low temperature.",
            Icon::TemperatureQuarter => {
                "A thermometer one-quarter full, representing low temperature."
            }
            Icon::TemperatureThreeQuarters => {
                "A thermometer three-quarters full, representing high temperature."
            }
            Icon::TengeSign => "The symbol for the Kazakhstani tenge, indicating currency.",
            Icon::Tent => "A single tent, representing camping or temporary shelter.",
            Icon::TentArrowDownToLine => {
                "A tent with an arrow pointing down to a line, representing a campsite."
            }
            Icon::TentArrowLeftRight => {
                "A tent with arrows pointing left and right, indicating horizontal setup."
            }
            Icon::TentArrowTurnLeft => {
                "A tent with an arrow turning left, indicating directional setup."
            }
            Icon::TentArrowsDown => "A tent with arrows pointing down, indicating tent setup.",
            Icon::Tents => "Multiple tents, representing camping or temporary shelter.",
            Icon::Terminal => "A computer terminal, representing command line or coding.",
            Icon::TextHeight => "An icon indicating text height adjustment.",
            Icon::TextSlash => "Text with a slash, indicating no text.",
            Icon::TextWidth => "An icon indicating text width adjustment.",
            Icon::Thermometer => "A thermometer, representing temperature measurement.",
            Icon::ThumbsDown => "A thumbs-down gesture, indicating disapproval or dislike.",
            Icon::ThumbsUp => "A thumbs-up gesture, indicating approval or like.",
            Icon::Thumbtack => "A thumbtack, indicating pinned items or locations.",
            Icon::Ticket => "A ticket, representing admission or entry to an event.",
            Icon::TicketSimple => "A simple ticket, representing admission or entry.",
            Icon::Tiktok => "The logo of TikTok, a video-sharing social media platform.",
            Icon::Timeline => "A timeline, representing chronological events.",
            Icon::ToggleOff => "A switch in the off position, indicating deactivation.",
            Icon::ToggleOn => "A switch in the 'on' position, indicating activation.",
            Icon::Toilet => "A toilet, representing restrooms.",
            Icon::ToiletPaper => "A roll of toilet paper, indicating sanitation.",
            Icon::ToiletPaperSlash => {
                "A toilet paper roll with a slash, indicating no toilet paper."
            }
            Icon::ToiletPortable => "A portable toilet, representing outdoor facilities.",
            Icon::ToiletsPortable => {
                "Portable toilets, indicating temporary sanitation facilities."
            }
            Icon::Toolbox => "A toolbox, representing tools or repair.",
            Icon::Tooth => "A tooth, representing dentistry or oral health.",
            Icon::ToriiGate => "A torii gate, representing Japanese culture.",
            Icon::Tornado => "A tornado, representing severe weather.",
            Icon::TowerBroadcast => "A broadcast tower, representing media transmission.",
            Icon::TowerCell => "A cell tower, representing communication.",
            Icon::TowerObservation => "A tall observation tower, representing sightseeing.",
            Icon::Tractor => "A tractor, representing agriculture or farming.",
            Icon::Trademark => "A trademark symbol, representing brand or intellectual property.",
            Icon::TrafficLight => "A traffic light, representing road signals.",
            Icon::Trailer => "A trailer, representing cargo or transport.",
            Icon::Train => "A train, representing railway transport.",
            Icon::TrainSubway => "A subway train, representing underground transportation.",
            Icon::TrainTram => "A tram, representing public transportation.",
            Icon::Transgender => {
                "A combined male and female symbol, representing transgender identity."
            }
            Icon::Trash => "A trash can, representing deletion or garbage.",
            Icon::TrashCan => "A trash can, representing waste disposal.",
            Icon::Tree => "A tree, representing nature or the environment.",
            Icon::TreeCity => "A tree with a cityscape, representing urban nature or parks.",
            Icon::TriangleExclamation => {
                "A triangle with an exclamation mark, indicating warning or caution."
            }
            Icon::Trophy => "A trophy, representing achievement or awards.",
            Icon::Trowel => "A trowel, representing construction or gardening.",
            Icon::TrowelBricks => "A trowel with bricks, representing construction or masonry.",
            Icon::Truck => "An icon of a truck, indicating transportation or delivery.",
            Icon::TruckArrowRight => "A truck with an arrow pointing right, representing delivery.",
            Icon::TruckDroplet => "A truck with a droplet, representing liquid transport.",
            Icon::TruckFast => "A fast-moving truck, indicating quick delivery or shipment.",
            Icon::TruckField => "A truck in a field, indicating agricultural transport.",
            Icon::TruckFieldUn => {
                "A truck in a field, indicating agricultural transport, with UN letters on it."
            }
            Icon::TruckFront => "A front view of a truck, indicating transportation or delivery.",
            Icon::TruckMedical => "A medical truck, representing emergency medical transport.",
            Icon::TruckMonster => "A monster truck, indicating a large, powerful vehicle.",
            Icon::TruckMoving => "A moving truck, representing relocation or transport.",
            Icon::TruckPickup => "A pickup truck, representing a vehicle or transportation.",
            Icon::TruckPlane => "A truck with a plane, representing logistics.",
            Icon::TruckRampBox => "A truck with a ramp, indicating delivery or loading.",
            Icon::Tty => "An old-fashioned telephone with a keyboard, indicating teletype.",
            Icon::TurkishLiraSign => "The symbol for the Turkish lira, indicating currency.",
            Icon::TurnDown => "An arrow curving downwards, indicating turning down.",
            Icon::TurnUp => "An arrow curving upwards, indicating turning up.",
            Icon::Tv => "A television set, representing media or entertainment.",
            Icon::Twitch => "The logo of Twitch, a live streaming platform.",
            Icon::Twitter => "The logo of Twitter, a well-known social media platform.",
            Icon::U => "The letter \"U\", representing the alphabet.",
            Icon::Umbrella => "An umbrella, indicating protection from rain or sun.",
            Icon::UmbrellaBeach => "An umbrella on a beach, representing leisure or vacation.",
            Icon::Underline => "A line below text, indicating underline or emphasis.",
            Icon::UniversalAccess => "A circle with a person inside, representing accessibility.",
            Icon::Unlock => "An open padlock, indicating access or security.",
            Icon::UnlockKeyhole => "An open padlock with a keyhole, indicating access or security.",
            Icon::Unsplash => "The logo of Unsplash, representing the photo sharing platform.",
            Icon::UpDown => {
                "An arrow pointing up and another pointing down, indicating vertical movement."
            }
            Icon::UpDownLeftRight => {
                "Arrows pointing in all four directions, indicating omnidirectional movement."
            }
            Icon::UpLong => "A long arrow pointing up, indicating upward direction or increase.",
            Icon::UpRightAndDownLeftFromCenter => {
                "An arrow pointing up right and down left from center, indicating movement."
            }
            Icon::UpRightFromSquare => {
                "An arrow pointing up and right from a square, indicating expansion or exit."
            }
            Icon::Upload => "An arrow pointing upward from a box, indicating upload.",
            Icon::User => "An outline of a person, indicating a user or profile.",
            Icon::UserAstronaut => {
                "A user icon wearing an astronaut helmet, representing an astronaut or space exploration."
            }
            Icon::UserCheck => {
                "A user icon with a check mark, indicating user approval or verification."
            }
            Icon::UserClock => {
                "A user icon with a clock, indicating user schedule or time management."
            }
            Icon::UserDoctor => {
                "A user icon with a stethoscope, representing a doctor or healthcare professional."
            }
            Icon::UserGear => "A user icon with a gear, representing user settings or management.",
            Icon::UserGraduate => {
                "A user wearing a graduation cap, indicating education or graduation."
            }
            Icon::UserGroup => "Multiple user icons, indicating a group or community.",
            Icon::UserInjured => "A user with an injury, representing injury or accident.",
            Icon::UserLarge => "A large user icon, indicating a prominent user.",
            Icon::UserLargeSlash => {
                "A large user icon with a slash, representing user removal or restriction."
            }
            Icon::UserLock => "A user with a lock, representing account security.",
            Icon::UserMinus => "A user icon with a minus sign, indicating user removal.",
            Icon::UserNinja => {
                "A user icon with a ninja mask, indicating a stealthy or anonymous user."
            }
            Icon::UserNurse => "A user with a nurse hat, representing medical staff.",
            Icon::UserPen => "A user icon with a pen, representing user editing or writing.",
            Icon::UserPlus => "A user icon with a plus sign, indicating adding a user.",
            Icon::UserSecret => {
                "A user with a finger over their lips, indicating secrecy or confidentiality."
            }
            Icon::UserShield => "A user with a shield, representing user protection or security.",
            Icon::UserSlash => "A user icon with a slash, indicating a removed or blocked user.",
            Icon::UserTag => "A user with a tag, representing user identification.",
            Icon::UserTie => "A user icon with a tie, indicating a professional user.",
            Icon::UserXmark => "A user with a circled X, representing user removal.",
            Icon::Users => "Multiple user icons, representing a group or community.",
            Icon::UsersBetweenLines => {
                "Multiple user icons between lines, indicating collaboration or communication."
            }
            Icon::UsersGear => {
                "Multiple user icons with a gear, representing user settings or management."
            }
            Icon::UsersLine => "Multiple users in a line, representing a group or queue.",
            Icon::UsersRays => "Multiple users with rays, representing community or influence.",
            Icon::UsersRectangle => {
                "Multiple user icons inside a rectangle, representing a group or community."
            }
            Icon::UsersSlash => "Multiple user icons with a slash, indicating no users or blocked.",
            Icon::UsersViewfinder => {
                "A user icon inside a viewfinder, representing focus on users."
            }
            Icon::Utensils => "A fork and knife, representing dining or food.",
            Icon::V => "A capital letter 'V', representing the letter.",
            Icon::VanShuttle => "A shuttle van, representing transportation.",
            Icon::Vault => "A vault, representing security or storage.",
            Icon::VectorSquare => "A square with vector points, representing design or graphics.",
            Icon::Venus => "The symbol of Venus, representing the female gender.",
            Icon::VenusDouble => "Two Venus symbols, representing female gender or partnership.",
            Icon::VenusMars => "The symbols of Venus and Mars combined, representing gender.",
            Icon::Vest => "A vest, representing clothing.",
            Icon::VestPatches => "A vest with patches, representing protective gear.",
            Icon::Vial => "A vial, representing a small container for liquids.",
            Icon::VialCircleCheck => {
                "A vial with a circled check mark, representing approved substance."
            }
            Icon::VialVirus => "A vial with a virus, representing medical testing.",
            Icon::Vials => "Two laboratory vials, representing testing or experimentation.",
            Icon::Video => "A video camera, indicating video content or recording.",
            Icon::VideoSlash => {
                "A video symbol with a slash, indicating no video or disabled video."
            }
            Icon::Vihara => "A Buddhist temple, representing a place of worship.",
            Icon::Virus => "A virus, representing infection or disease.",
            Icon::VirusCovid => "A representation of the COVID-19 virus.",
            Icon::VirusCovidSlash => {
                "A virus symbol with a slash, representing COVID-19 eradication."
            }
            Icon::VirusSlash => "A virus with a slash, indicating antivirus or no virus.",
            Icon::Viruses => "Multiple virus icons, representing infections or disease.",
            Icon::Voicemail => "An icon of a cassette tape, representing voicemail messages.",
            Icon::Volcano => "A volcano, representing eruption or natural phenomenon.",
            Icon::Volleyball => "A volleyball, representing the sport.",
            Icon::VolumeHigh => "A speaker with high volume, representing loud audio.",
            Icon::VolumeLow => "A speaker with low volume, representing soft audio.",
            Icon::VolumeOff => "A speaker without sound waves, indicating no volume.",
            Icon::VolumeXmark => "A speaker with an `X`, indicating mute or no sound.",
            Icon::VrCardboard => "The logo of Google Cardboard, a VR platform.",
            Icon::W => {
                "A capital letter 'W', representing the letter or something starting with W."
            }
            Icon::WalkieTalkie => "A walkie-talkie, representing communication devices.",
            Icon::Wallet => "A wallet, representing money or finances.",
            Icon::WandMagic => "A magic wand, representing magical effects.",
            Icon::WandMagicSparkles => {
                "A magic wand with sparkles, indicating magical effects or settings."
            }
            Icon::WandSparkles => "A magic wand with sparkles, indicating magic or settings.",
            Icon::Warehouse => "A warehouse building, representing storage or logistics.",
            Icon::Water => "A water droplet, representing liquid or hydration.",
            Icon::WaterLadder => "A water ladder, representing swimming pools or rescue.",
            Icon::WaveSquare => "A square wave, representing a waveform.",
            Icon::WebAwesome => "The logo of Web Awesome, representing the web development tool.",
            Icon::WeightHanging => "A hanging weight, representing heavy lifting or measurement.",
            Icon::WeightScale => "A weight scale, representing measurement of weight.",
            Icon::WheatAwn => "A stalk of wheat, representing grain or agriculture.",
            Icon::WheatAwnCircleExclamation => {
                "A circle with wheat and an exclamation mark, representing gluten alert."
            }
            Icon::Wheelchair => "A simple wheelchair, representing accessibility.",
            Icon::WheelchairMove => "A wheelchair with motion lines, representing mobility.",
            Icon::WhiskeyGlass => "A whiskey glass, representing alcohol or beverages.",
            Icon::Wifi => "A signal icon, representing wireless internet connectivity.",
            Icon::Wind => "A wind symbol, representing breeze or weather.",
            Icon::WindowMaximize => "A window being maximized, representing expansion.",
            Icon::WindowMinimize => "A window being minimized, representing reduction.",
            Icon::WindowRestore => "A window being restored, representing reopening or resizing.",
            Icon::Windows => "The logo of Windows, a popular operating system by Microsoft.",
            Icon::WineBottle => "A wine bottle, representing alcohol or beverages.",
            Icon::WineGlass => "A wine glass, representing drinking or celebration.",
            Icon::WineGlassEmpty => "An empty wine glass, representing a drink.",
            Icon::WizardsOfTheCoast => {
                "The logo of Wizards of the Coast, representing the game company."
            }
            Icon::WonSign => "The symbol for the South Korean won, indicating currency.",
            Icon::Wordpress => "The logo of WordPress, a popular content management system.",
            Icon::Worm => "A worm, representing the animal or an insult.",
            Icon::Wrench => "A wrench, representing tools or repair.",
            Icon::X => "A capital letter `X`, representing the letter or close.",
            Icon::XRay => "An X-ray, representing medical imaging.",
            Icon::Xbox => "The logo of Xbox, representing the gaming console.",
            Icon::Xmark => "A simple `X` mark, indicating error or cancellation.",
            Icon::XmarksLines => "Lines forming an `X`, representing rejection or closure.",
            Icon::Y => "The letter `Y`, representing the alphabet.",
            Icon::YenSign => "The symbol for the Japanese yen, indicating currency.",
            Icon::YinYang => "A yin-yang symbol, representing balance or duality.",
            Icon::Youtube => "The logo of YouTube, a video-sharing platform.",
            Icon::Z => "A capital letter 'Z', representing the letter or sleep.",
        }
    }

    /// Writes the Font Awesome icons set to a csv.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the csv file.
    ///
    /// # Errors
    ///
    /// * If the file cannot be created or written to, an error is returned.
    pub fn to_csv(path: &str) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(path)?;

        for icon in Icon::iter() {
            let row = IconRow { name: icon.class(), description: icon.description() };
            writer.serialize(row)?;
        }

        writer.flush()
    }
}
