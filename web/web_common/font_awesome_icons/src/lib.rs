//! Library providing a Rust Font-Awesome enumeration.
mod class;
mod descriptions;
mod from_str;
pub mod diesel_impls;
pub mod errors;
mod as_ref;
mod try_from;

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[cfg(feature = "pgrx")]
use pgrx::FromDatum;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::FAIcon)
)]
/// Struct representing a Font Awesome icon.
pub enum FAIcon {
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
