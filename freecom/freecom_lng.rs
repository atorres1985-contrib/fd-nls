# $Id$
#
# FreeCOM national customization file
#
#  Language: Serbian / Yugoslavian
#  Codepage: 852
#  Author:   Dejan �trbac (strbac@phreaker.net)
#
# This file is used to generate all the messages that command.com
# outputs.  This file is the input to the fixstrs program, and it
# outputs strings.h and strings.dat.  The .DAT file is appended to
# the command.exe file, and then renamed to command.com.  The .H
# file contains all the info for the program to retreive the
# messages.
#
# The format of this file is simple.  Blank lines and lines starting
# with "#" are ignored.
# Each message starts with a label name that will be used to refer to
# the message in the program.  A label starts with a colon ":".
# A label has a version ID attached to it delimited by a hash sign, e.g.:
#  :TEXT_LABEL#1
# This version is incremented each time the contents of the string has
# undergo a larger change. The same language definition may contain the
# same label up to one time, regardless of the version. FIXSTRS compares
# both the label and the version ID and both must match.
# A missing version is assumed as "#0".
#
# All lines after the label are the message until a line with a
# single "." or "," in the first column.  The difference is the
# period (".") signifies that there will be a final carrage return when
# the message is displayed, but a comma does not.
#
# The body may contain backslash escape sequences as known from C; there
# are the usual \# (where '#' is a lowercase letter except 'x' or one of
# "[]{}?"), \\ (to embed a backslash itself)
# \x?? (where '??' are up to two hexadecimal digits), \0 (to embed a NUL
# character), \, and \. (to specify a period or comma in the first column
# of a line) and the single \ at the end of the line to suppress to append
# a newline character. Note: There is NO octal sequence except the short \0!
# There is a known bug (or feature): [ignore the very first hash mark]
#:TEXT_LABEL#2
#
#\
#,
# Although the first data line appends the newline, the second does not,
# though the comma removes the newline from the first line.

# Defining prompts
# Some prompts may cause an user interaction. Those ones should be in sync
# with the issued text. To define how to interprete a pressed key, they
# are mapped into metakeys like that: [ignore first hash sign]

## Return value: a -> Yes; else -> No
#:PROMPT_YES_NO#1
#YyNn\n\r{CBREAK}
#aabb b b       b
# (Yes/No) ? \
#.

# All strings, which label start with "PROMPT_", are parsed as prompts.
# The first two lines of the body are special; the first one enumerates all
# valid keys, the second one assigns arbitary metakeys. Metakeys may
# range from 'a' through 'z'; spaces are ignored; everything else cause
# an error. The comment preceeding the prompt definition associates the
# metakeys with their meaning.
# The remaining lines of the body contain the text to be displayed.
#
# Above example defines a native Yes/No prompt with a space behind the question
# mark and no appended newline.
# The metakey 'a' means "User answered with 'Yes'" and 'b' means no.
# The keys 'Y' and 'y' are mapped to metakey 'a' (aka Yes) and the keys
# 'N', 'n', Enter and ^Break are mapped to metakey 'b' (aka No).
# The spaces between the 'b's in the second line had been inserted to
# align them with the corresponding keys of the first line, hence in order
# to enhance readibility of the association between the pressed keys and their
# mapping into a metakey.
#
# The first line (pressed keys) has to enumerate the ASCII value as returned
# by DOS or BIOS (INT-10); special keys normally expressed with ASCII code zero,
# but a non-zero scancode are NOT supported; this limit includes for instance
# the function keys F1 through F12 and the cursor keys and it is not possible
# to differ between the number pad and normal keys.
# The keys may be enumerated by their ASCII character, by a backslash sequence,
# or a symbolic name enclosed in curly brackets (see FIXSTRS.C "symkeys[]"
# array about the supported symnames).


#
#  These are error messages
#
## Issued if a single character option is unknown
:TEXT_ERROR_INVALID_SWITCH
Neva�e�i prekida� - /%c
.

## Issued if a longname option is unknown
:TEXT_ERROR_INVALID_LSWITCH
Neva�e�i prekida� - /%s
.

## Issued if the context, the type of argument etc. is invalid
:TEXT_ERROR_ILLFORMED_OPTION
Lo�e formirana opcija: '%s'
.

:TEXT_ERROR_OPT_ARG
Opcija '%s' ne mo�e imati argument.
.

:TEXT_ERROR_OPT_NOARG
Opcija '%s' mora imati argument.
.

:TEXT_INVALID_NUMBER
Neva�e�i broj naveden u '%s'
.

:TEXT_ERROR_CLOSE_QUOTE
Nedostaje krajnji navodnik: %c
.

:TEXT_ERROR_TEMPFILE
Privremeni fajl ne mo�e biti kreiran.
.

:TEXT_ERROR_TOO_MANY_PARAMETERS_STR
Isuvi�e parametara - '%s'
.

:TEXT_ERROR_TOO_MANY_PARAMETERS
Isuvi�e parametara.
.

:TEXT_ERROR_INVALID_PARAMETER
Neva�e�i parametar - '%s'
.

:TEXT_ERROR_PATH_NOT_FOUND
Putanja nije prona�ena.
.

:TEXT_ERROR_FILE_NOT_FOUND
Fajl nije prona�en.
.

:TEXT_ERROR_SFILE_NOT_FOUND
Fajl nije prona�en - '%s'
.

:TEXT_ERROR_REQ_PARAM_MISSING
Potreban parametar nedostaje.
.

:TEXT_ERROR_INVALID_DRIVE
Neva�e�i drajv - %c:
.

:TEXT_ERROR_BADCOMMAND#2
Lo�a komanda ili naziv fajla - "%s".
.

:TEXT_ERROR_OUT_OF_MEMORY
Gre�ka nedostatka memorije.
.

:TEXT_ERROR_OUT_OF_DOS_MEMORY#1
Alokacija DOS memorije nije uspela.
.

:TEXT_ERROR_CANNOTPIPE
Nemogu�nost cevovoda! Privremeni fajl ne mo�e biti otvoren!
.

:TEXT_ERROR_LONG_LINE_BATCHFILE
Linija #%ld u batch fajlu '%s' je preduga.
.

:TEXT_ERROR_BFILE_VANISHED
Batch fajl '%s' nije prona�en.
.

:TEXT_ERROR_BFILE_LABEL
Batch fajl '%s' ne sadr�i labelu '%s'.
.

:TEXT_ERROR_DIRFCT_FAILED#1
%s ne uspeva za '%s'.
.
# The next three errors must remain in this order!
:TEXT_ERROR_SET_ENV_VAR
Promenljiva okru�enja '%s' ne mo�e biti postavljena.
Okru�enje prepuno?
.
:TEXT_ERROR_ENV_VAR_NOT_FOUND
Promenljiva okru�enja '%s' nije prona�ena.
.
:TEXT_ERROR_NO_ENVIRONMENT
Nema okru�enja. Mogu�i nedostatak memorije. Navedite /E opciju.
.

# The next three errors must remain in this order!
:TEXT_ERROR_SET_ALIAS#1
Alias '%s' ne mo�e biti postavljen. Prostor za aliase prepun?
.
:TEXT_ERROR_ALIAS_NOT_FOUND#1
Alias '%s' nije prona�en.
.
:TEXT_ERROR_NO_ALIAS_SEGMENT#1
Nema prostora za aliase. Mogu�i nedostatak memorije.
.

:TEXT_ERROR_SYNTAX_STR
Sintaksna gre�ka - '%s'
.

:TEXT_ERROR_SYNTAX
Sintaksna gre�ka.
.

:TEXT_ERROR_FILENAME_TOO_LONG
Naziv fajla je preduga�ak - '%s'
.

:TEXT_ERROR_SELFCOPY
Fajl '%s' ne mo�e biti kopiran u sebe.
.

:TEXT_ERROR_COMMAND_TOO_LONG
Komandna linija je preduga posle razvijanja aliasa!
.

:TEXT_ERROR_LINE_TOO_LONG
Linija je preduga. Komanda ne mo�e biti izvr�ena.
.

:TEXT_ERROR_HISTORY_SIZE#1
Neva�e�a veli�ina istorije - '%s'
.

:TEXT_HISTORY_EMPTY#1
Istorija komandne linije je prazna.
.

:TEXT_ERROR_BAD_MCB_CHAIN
MCB lanac je o�te�en, ili MS-DOS nekompatibilan sistem.
.

:TEXT_ERROR_UNDEFINED_ERROR
Nedefinisana gre�ka %d.
.

:TEXT_ERROR_REGION_WARNING
Neva�e�e memorijsko podru�je %d - ignorisano.
.

:TEXT_ERROR_ON_OR_OFF
Morate navesti ON ili OFF.
.

:TEXT_ERROR_BAD_VARIABLE
Lo�e navedena promenljiva.
.

:TEXT_ERROR_IN_MISSING#1
FOR: nedostaje IN
.

:TEXT_ERROR_MISSING_PARENTHESES#1
Jedna ili obe zagrade nedostaju.
.

:TEXT_ERROR_DO_MISSING#1
FOR: nedostaje DO
.

:TEXT_ERROR_NO_COMMAND_AFTER_DO#1
FOR: Nedostaje komanda posle DO iskaza.
.

:TEXT_ERROR_REDIRECT_FROM_FILE
Ulaz ne mo�e biti preusmeren na fajl '%s'.
.

:TEXT_ERROR_REDIRECT_TO_FILE
Izlaz ne mo�e biti preusmeren na fajl '%s'.
.

:TEXT_ERROR_EMPTY_REDIRECTION#1
Prazno preusmerenje.
.

:TEXT_ERROR_INVALID_DATE
Neva�e�i datum.
.

:TEXT_ERROR_INVALID_TIME
Neva�e�e vreme.
.

:TEXT_ERROR_NO_GOTO_LABEL
Nema labele navedene za GOTO.
.

:TEXT_CTTY_NOTIMPLEMENTED
Komanda CTTY je izostavljena iz ovog COMMAND.COMa.
.

:TEXT_ERROR_NORW_DEVICE
Neva�e�i ili ne postoje�i u�itno-upisni ure�aj '%s'.
.

:TEXT_ERROR_CTTY_DUP
Neuspeh pri promeni fajl deskriptora na TTY '%s'.
.

:TEXT_ERROR_L_NOTIMPLEMENTED
Prekida� /L nije implementiran.
.

:TEXT_ERROR_U_NOTIMPLEMENTED
Prekida� /U nije implementiran.
.

:TEXT_ERROR_WRITING_DEST
Gre�ka pri upisu na odredi�te.
.

:TEXT_ERROR_CANNOT_OPEN_SOURCE
Izvor nije mogao biti otvoren - '%s'
.

:TEXT_ERROR_OPEN_FILE
Fajl '%s' nije mogao biti otvoren.
.

:TEXT_ERROR_READ_FILE
Nemogu�nost �itanja iz fajla: '%s'
.

:TEXT_ERROR_WRITE_FILE
Nemogu�nost upisa u fajl: '%s'
.

:TEXT_ERROR_LEADING_PLUS
Povezuju�i znak '+' ne mo�e voditi argumente.
.

:TEXT_ERROR_TRAILING_PLUS
Povezuju�i znak '+' ne mo�e pratiti argumente.
.

:TEXT_ERROR_NOTHING_TO_DO
Nema ni�eg da se uradi.
.

:TEXT_ERROR_COPY
COPY komanda nije uspela.
.

:TEXT_ERROR_IF_EXIST_NO_FILENAME#1
IF EXIST: nedostaje naziv fajla
.
:TEXT_ERROR_IF_ERRORLEVEL_NO_NUMBER#1
IF ERRORLEVEL: nedostaje broj
.
:TEXT_ERROR_IF_ERRORLEVEL_INVALID_NUMBER#1
IF ERRORLEVEL: neva�e�i broj
.
:TEXT_ERROR_IF_MISSING_COMMAND#1
IF: nedostaje komanda
.
:TEXT_NOT_IMPLEMENTED_YET
�ao nam je. Nije implementirano.
.

:TEXT_FAILED_LOAD_STRINGS
Neuspeh pri u�itavanju poruka u memoriju.
.

:TEXT_MSG_NOTIMPLEMENTED
Prekida� /MSG nije implementiran.
.

:TEXT_MSG_ITEMS_DISPLAYED#1
Ukupno izlistano stavki: %u
.

:TEXT_CORRUPT_COMMAND_LINE
O�te�ena komandna linija. Ovo je interna gre�ka i povezana je sa
sistemom u kome radi COMMAND.COM. Molimo vas prijavite ovu gre�ku.
.

:TEXT_QUOTED_C_OR_K#1
Opcije /C i /K nemogu biti pod navodnicima, ignorisane.
.

:TEXT_INIT_FULLY_QUALIFIED#1
Putanja do COMMAND.COMa mora biti potpuna!
Ovo zna�i uklju�uju�i slovo drajva po�inju�i obrnutom kosom crtom (\\).
Na primer: C:\\

COMMAND.COM sada podrazumeva ovu putanju:
%s
.

:TEXT_ERROR_RESTORE_SESSION
Informacije o sesiji nisu mogle biti vra�ene. Sve lokalne postavke
su izgubljene. Molimo vas pogledajte gore izjavljene poruke o gre�kama
za mogu�i razlog nastanka ovog problema.
.

:TEXT_ERROR_SAVE_SESSION
Trenutne informacije nemogu biti o�uvane za vreme poziva programa.
Molimo vas pogledajte gore izjavljene poruke o gre�kama za mogu�i razlog
nastanka ovog problema.
.

:TEXT_ERROR_CWD_FAILED
Drajv %c: ne reaguje ili nedostatak interne memorije.
.

:TEXT_ERROR_KSWAP_ALIAS_SIZE
Swapping nije uspeo: Aliasi zauzimaju previ�e memorije.
.


:TEXT_ERROR_KSWAP_ALLOCMEM
Swapping nije uspeo: Daleka memorija ne mo�e biti alocirana.
.

:TEXT_ERROR_ALIAS_OUT_OF_MEM#1
Nedostatak prostora za aliase.
.

:TEXT_ERROR_ALIAS_NO_SUCH#1
Alias '%s' ne postoji.
.

:TEXT_ERROR_ALIAS_INSERT#1
Gre�ka pri umetanju aliasa.
.

:TEXT_ALIAS_INVALID_NAME#1
Neva�e�e ime aliasa '%s'.
.

:TEXT_ERROR_LOADING_CONTEXT#1
Manipulator kriti�nih gre�aka ili kontekst modul ne mo�e biti u�itan.
.

:TEXT_ERROR_CONTEXT_OUT_OF_MEMORY#1
Nedostatak memorije za kontekst.
Ako ova gre�ka nastavi da se javlja, razmotrite pove�anje internog
bafera, kao istorije, steka direktorijuma itd.
.

:TEXT_ERROR_CONTEXT_LENGTH#1
Veli�ina konteksta prelazi maksimalni limit za %lu bajta.
Promena veli�ine konteksta na %u bajta.
.

:TEXT_ERROR_CONTEXT_ADD_STATUS#1
Neuspeh pri dodavanju statusnih informacija u kontekst. Ova gre�ka
mo�e zna�iti o�te�enje memorije ili nepravilno utvr�ena minimalna
veli�ina konteksta.
.

:TEXT_ERROR_CONTEXT_AFTER_SWAP#1
Kontekst nedostaje posle swapa. Kontekst je rekreiran, ali su svi
aliasi itd. izgubljeni.
.

#
# Informational messages
#

:TEXT_MSG_PAUSE#1
Pritisnite bilo koji taster za nastavak...\
.

:TEXT_MSG_HISTORY_SIZE
Veli�ina istorije je %d bajta.
.

:TEXT_MSG_DOSKEY
DOSKEY je ve� omogu�en interno u komandnoj ljusci.
Pokrenite DOSKEY /? za vi�e informacija o implementaciji.
.

:TEXT_MSG_ECHO_STATE
ECHO je %s
.

:TEXT_MSG_VERIFY_STATE
VERIFY je %s
.

:TEXT_MSG_FDDEBUG_STATE
DEBUG izlaz je %s
.
:TEXT_MSG_FDDEBUG_TARGET
DEBUG izlaz se prikazuje na '%s'
.

:TEXT_MSG_BREAK_STATE
BREAK je %s
.

:TEXT_MSG_CURRENT_DATE
Trenutni datum je: %s
.

## The three DATE prompts MUST be in this order!
:TEXT_MSG_ENTER_DATE_AMERICAN#1
Unesite novi datum (mm%sdd%s[vv]gg): \
.
:TEXT_MSG_ENTER_DATE_EUROPE#1
Unesite novi datum (dd%smm%s[vv]gg): \
.
:TEXT_MSG_ENTER_DATE_JAPANESE#1
Unesite novi datum ([vv]gg%smm%sdd): \
.

:TEXT_MSG_CURRENT_TIME
Trenutno vreme je: %s
.

:TEXT_STRING_PM#1
 pm\
.
:TEXT_STRING_AM#1
 am\
.

:TEXT_MSG_ENTER_TIME#1
Unesite novo vreme: \
.

# src-file <operation> target-file
:TEXT_MSG_COPYING
%s %s %s
.

# This prompt MUST include the pseudo key CBREAK!
# Note: This prompt ignores DOS NLS intentionally in order to
# keep interactive prompt & user-interaction in sync.
# Used by Delete all (Y/N) --> let ENTER default to NO
# Return value: a -> Yes; else -> No
:PROMPT_DELETE_ALL#1
DdNn{CR}{LF}{CBREAK}
aabb   b   b       b
Svi fajlovi u direktorijumu '%s' �e biti obrisani!
Da li ste sigurni (Da/Ne)? \
.

# This prompt MUST include the pseudo key CBREAK!
# Note: This prompt ignores DOS NLS intentionally in order to
# keep interactive prompt & user-interaction in sync.
# Return value: a -> Yes; else -> No
:PROMPT_YES_NO#1
DdNn{LF}{CR}{CBREAK}{ESC}
aabb   a   a       b    b
 [ENTER=Da, ESC=Ne]? \
.

# This prompt MUST include the pseudo key CBREAK!
# Note: This prompt ignores DOS NLS intentionally in order to
# keep interactive prompt & user-interaction in sync.
# Attention: This prompt is issued via BIOS; any newline MUST be prefixed
#  by \r!
# Return value: a -> Yes; b -> No; c -> All; else -> Undefined
:PROMPT_CANCEL_BATCH#1
DdNnSs{LF}{CR}{CBREAK}{ESC}
aabbcc   a   a       c    b
Control-Break pritisnuto.\r
Prekinuti batch fajl '%s' (Da/Ne/Sve)? \
.

# This prompt MUST include the pseudo key CBREAK!
# Note: This prompt ignores DOS NLS intentionally in order to
# keep interactive prompt & user-interaction in sync.
# Return value: a -> Yes; b -> No; c -> All; d -> Quit
:PROMPT_OVERWRITE_FILE#1
DdNnSsOo{BREAK}{ENTER}{ESC}
aabbccdd      d      a    b
Pisati preko '%s' (Da/Ne/Svi/Otkaz)? \
.

# This prompt MUST include the pseudo key CBREAK!
# Note: This prompt ignores DOS NLS intentionally in order to
# keep interactive prompt & user-interaction in sync.
# Return value: a -> Yes; b -> No; c -> All; d -> Quit
:PROMPT_DELETE_FILE#1
DdNnSsOo{BREAK}{ENTER}{ESC}
aabbccdd      d      a    b
Obrisati '%s' (Da/Ne/Svi/Otkaz)? \
.

:TEXT_UNKNOWN_FILENAME#1
<<nepoznato>>\
.

:TEXT_DIRSTACK_EMPTY
Stek direktorijuma je prazan.
.

## Strings to construct the DIR output
:TEXT_DIR_HDR_VOLUME#1
 Volumen u drajvu %c \
.
:TEXT_DIR_HDR_VOLUME_STRING
je %s
.
:TEXT_DIR_HDR_VOLUME_NONE
nema labele
.
:TEXT_DIR_HDR_SERIAL_NUMBER
 Serijski broj volumena je %04X-%04X
.
:TEXT_DIR_FTR_FILES#1
%10s fajl(a)\
.
:TEXT_DIR_FTR_BYTES
   %12s bajta
.
:TEXT_DIR_FTR_TOTAL_NUMBER
Ukupno prikazano:
.
:TEXT_DIR_FTR_DIRS#1
%10s dir(a)\
.
:TEXT_DIR_FTR_BYTES_FREE
 %15s bajta slobodno
.
:TEXT_DIR_DIRECTORY
Direktorijum %s
.
:TEXT_DIR_DIRECTORY_WITH_SPACE
 Direktorijum %s
.
:TEXT_DIR_LINE_FILENAME_WIDE#1
%-15s\
.
:TEXT_DIR_LINE_FILENAME_BARE
%-13s
.
:TEXT_DIR_LINE_FILENAME_SINGLE#1
%-13s\
.
:TEXT_DIR_LINE_FILENAME#1
%-8s %-3s \
.
:TEXT_DIR_LINE_SIZE_DIR#1
        <DIR> \
.
:TEXT_DIR_LINE_SIZE#1
   %10s \
.

:TEXT_FILE_COMPLATION_DISPLAY#1
%-14s\
.

:TEXT_MSG_PATH
PATH=%s
.
:TEXT_MSG_PATH_NONE#1
Putanja nije definisana.
.

## The following names MUST be in this order!
:TEXT_WEEKDAY_SHORT_NAME_SUNDAY#1
Ned\
.
:TEXT_WEEKDAY_SHORT_NAME_MONDAY#1
Pon\
.
:TEXT_WEEKDAY_SHORT_NAME_TUESDAY#1
Uto\
.
:TEXT_WEEKDAY_SHORT_NAME_WEDNSDAY#1
Sre\
.
:TEXT_WEEKDAY_SHORT_NAME_THURSDAY#1
�et\
.
:TEXT_WEEKDAY_SHORT_NAME_FRIDAY#1
Pet\
.
:TEXT_WEEKDAY_SHORT_NAME_SATURDAY#1
Sub\
.

# Displayed by DEL how many files were removed.
# These three strings must be kept in order!
:TEXT_MSG_DEL_CNT_FILES#1
Nijedan fajl nije izbrisan.
.
:TEXT_MSG_DEL_CNT_FILES_1#1
Jedan fajl izbrisan.
.
:TEXT_MSG_DEL_CNT_FILES_2#1
%u fajla izbrisano.
.

:TEXT_MSG_SHOWCMD_INTERNAL_COMMANDS
Dostupne interne komande:

.

:TEXT_MSG_SHOWCMD_FEATURES
.

## Displayed within "?" <-> showcmd() to enumerate the included features
## Note the trailing single space
:TEXT_SHOWCMD_FEATURE_ALIASES#1
.
:TEXT_SHOWCMD_FEATURE_ENHANCED_INPUT#1
.
:TEXT_SHOWCMD_FEATURE_HISTORY#1
.
:TEXT_SHOWCMD_FEATURE_FILENAME_COMPLETION#1
.
:TEXT_SHOWCMD_FEATURE_SWAP_EXEC#1
.
:TEXT_SHOWCMD_FEATURE_CALL_LOGGING#1
.
:TEXT_SHOWCMD_FEATURE_LAST_DIR#1
.
:TEXT_SHOWCMD_FEATURE_KERNEL_SWAP_SHELL#1
.
:TEXT_SHOWCMD_FEATURE_XMS_SWAP#1
.
:TEXT_SHOWCMD_DEFAULT_TO_SWAP#1
.
:TEXT_SHOWCMD_FEATURE_INSTALLABLE_COMMANDS#1
.
:TEXT_SHOWCMD_FEATURE_NLS#1
.
:TEXT_SHOWCMD_FEATURE_DIRSTACK#1
.
:TEXT_SHOWCMD_FEATURE_DEBUG#1
.

:TEXT_MSG_INIT_BYPASS_AUTOEXEC#1

Pritisnite F8 za mod pracenja, ili F5 za preskakanje %s... \
.
:TEXT_MSG_INIT_BYPASSING_AUTOEXEC
Preskakanje preko '%s'
.

:TEXT_MSG_VER_DOS_VERSION
DOS verzija %u.%u
.

:TEXT_MSG_VER_EARLY_FREEDOS
FreeDOS jezgro (gradnja 1933 ili ranija)
.

:TEXT_MSG_VER_LATER_FREEDOS
FreeDOS jezgro %d.%d.%d
.


# Displayed when the shell is to terminate, but has been started
# with /P option <-> shell cannot exist;
# This is a crash situation, because FreeCOM won't reach this situation
# normally otherwise
# All newlines must be prefixed by \r's !
:TEXT_MSG_REBOOT_NOW#1
\r\n\r
Ljuska �e sada biti prekinuta, iako je ovo zabranjeno\r
(obi�no navo�enjem prekida�a "/P").\r
Morate restartovati sistem ili, ukoliko ova ljuska radi u\r
vi�eprocesnom okru�enju, prekinuti ovaj proces.\r
.

# Displayed during the initialization phase of FreeCOM, if its own
# filename could not be determined.
:TEXT_MSG_FREECOM_NOT_FOUND#1
Izvrsni fajl komandne ljuske nije pronadjen.
Morate navesti potpunu putanju do COMMAND.COMa kao
prvi argument COMMANDa, na primer:
C:\\
.


:TEXT_MEMORY_ENVIRONMENT#1
NA %5u %5u
.
:TEXT_MEMORY_CONTEXT#1
NA %5u %5u
.
:TEXT_MEMORY_HEAP#1
NA %5lu
.
:TEXT_MEMORY_CTXT_ALIAS#1
NA %5u %5u %5u
.
:TEXT_MEMORY_CTXT_HISTORY#1
NA %5u %5u %5u
.
:TEXT_MEMORY_CTXT_DIRSTACK#1
NA %5u %5u %5u
.
:TEXT_MEMORY_CTXT_LASTDIR#1
NA %5u %5u
.
:TEXT_MEMORY_CTXT_BATCH#1
NA %5u %5u
.
:TEXT_MEMORY_CTXT_SWAPINFO#1
NA %5u %5u
.


## CHCP
:TEXT_ERROR_GET_CODEPAGE#1
Neuspeh pri dobavljanju trenutne kodne strane iz sistema.
.
:TEXT_ERROR_SET_CODEPAGE#1
Neuspeh pri promeni trenutne kodne strane.
.
:TEXT_DISPLAY_CODEPAGE#1
Trenutna kodna strana je: %u.
Sistemska kodna strana (pravilno) je: %u.
.

#
# Command help text
#

:TEXT_CMDHELP_ALIAS
Prikazuje, postavlja, ili uklanja aliase.

  ALIAS [promenljiva[=][string]]

  promenljiva  Navodi ime aliasa.
  string       Navodi niz karaktera za dodelu aliasu.

 Pokrenite ALIAS bez parametara za prikaz trenutnih aliasa. Ukoliko string
 nije naveden, postoje�i alias se uklanja.
.

:TEXT_CMDHELP_BEEP
Stvara kratak zvuk upozorenja.

  BEEP
.

:TEXT_CMDHELP_BREAK
Postavlja ili uklanja pro�irenu CTRL+C proveru.

  BREAK [ON | OFF]

 Pokrenite BREAK bez parametara za prikaz trenutne postavke.
.

:TEXT_CMDHELP_CALL#1
Poziva jedan batch program iz drugog.

  CALL [/S | /N] [/Y] [drajv:][putanja]fajl [batch-parametri]

  batch-parametri   Navodi informacije sa komandne linije potrebne
                    batch programu.

  /S uklju�uje dok /N isklju�uje swapping komandne ljuske.
  /Y omogu�ava mod pra�enja prilikom izvr�enja komande.
.

:TEXT_CMDHELP_CD
Prikazuje naziv ili menja trenutni direktorijum.

  CHDIR [drajv:][putanja]
  CHDIR[..]
  CD [drajv:][putanja]
  CD[..]
  CD -

  ".."  Navodi da �elite da promenite direktorijum u roditeljski trenutnog.
  "-"   Vra�a se u prethodni direktorijum.

 Pokrenite "CD drajv:" za prikaz trenutnog dira na navedenom drajvu.
 Pokrenite CD bez parametara za prikaz trenutnog drajva i direktorijuma.
 Tako�e pogledajte komandu CDD.
.

:TEXT_CMDHELP_CDD
Prikazuje trenutni direktorijum ili menja trenutni direktorijum i drajv.

  CDD [drajv:][putanja]
  CDD[..]
  CDD -

  ".."  Navodi da �elite da promenite direktorijum u roditeljski trenutnog.
  "-"   Vra�a se u prethodni direktorijum.

 Ako je "drajv:" naveden, trenutni radni drajv se menja na navedeni.
 Ovo je jedina razlika u odnosu na "CD" odnosno "CHDIR".
 Pokrenite CDD bez parametara za prikaz trenutnog drajva i direktorijuma.
.

:TEXT_CMDHELP_CHCP
Prikazuje ili postavlja broj aktivne kodne strane.

  CHCP [nnn]

  nnn   Navodi broj kodne strane.

 Pokrenite CHCP bez parametara za prikaz aktivne kodne strane.
.

:TEXT_CMDHELP_CLS
Bri�e ekran i postavlja standardne atribute boja.

  CLS
.

:TEXT_CMDHELP_COMMAND
Pokre�e novu instancu komandne ljuske.

  COMMAND [[drajv:]putanja] [ure�aj] [/E:nnnnn] [/P] [/MSG]
           [/LOW] [/Y [/[C|K] komanda]]

  [drajv:]putanja  Navodi direktorijum koji sadr�i COMMAND.COM.
  ure�aj           Navodi ure�aj za komandni ulaz/izlaz.
  /E:nnnnn         Postavlja po�etnu veli�inu okru�enja na nnnnn bajta.
                   (nnnnn treba biti broj izme�u 256 i 32,768).
  /P               Postavlja novu komandnu ljusku za stalnu (nema izlaza).
  /LOW             Primorava komandnu ljusku da dr�i svoje stalne podatke
                   u niskoj memoriji.
  /MSG             U�itava sve sistemske poruke u memoriju (zahteva i /P).
  /Y               Kora�a kroz batch program naveden sa /C ili /K.
  /C komanda       Izvr�ava navedenu komandu i vra�a se u trenutnu ljusku.
  /K komanda       Izvr�ava navedenu komandu i nastavlja rad.
.

:TEXT_CMDHELP_COPY
Kopira jedan ili vi�e fajlova na drugu lokaciju.

  COPY [/A | /B] izvor [/A | /B] [+ izvor [/A | /B] [+ ...]]
        [odredi�te [/A | /B]] [/V] [/Y | /-Y]

  izvor        Navodi fajl ili fajlove koje treba prekopirati.
  /A           Nagove�ta ASCII tekstualni fajl.
  /B           Nagove�ta binarni fajl.
  odredi�te    Navodi direktorijum i/ili naziv fajla za novi(e) fajl(ove).
  /V           Proverava korektnost upisa novih fajlova.
  /Y           Prigu�ava upite za pisanje preko ve� postoje�ih fajlova u
               odredi�tu, odgovaraju�i potvrdno.
  /-Y          Poziva upit za potvrdu pisanja preko ve� postoje�ih fajlova
               u odredi�tu.

 Prekida� /Y mo�e biti naveden u COPYCMD promenljivi okru�enja.
 Ovo mo�e biti poni�teno sa /-Y na komandnoj liniji.

 Za spajanje fajlova, navedite jedan fajl za odredi�te, ali vi�e fajlova za
 izvor (koriscenjem d�oker znakova ili fajl1+fajl2+fajl3 formatom).
.

:TEXT_CMDHELP_CTTY
Menja terminalni ure�aj kori�en za kontrolu sistema.

  CTTY ure�aj

  ure�aj    Terminalni ure�aj koji �elite koristiti, kao npr. COM1.
.

:TEXT_CMDHELP_DATE#1
Prikazuje ili postavlja datum.

  DATE [/D] [datum]

  /D odvra�a DATE od interaktivnosti.

 Pokrenite DATE bez parametara za prikaz trenutne postavke datuma
 i upit za novu. Pritisnite ENTER da zadr�ite isti datum.
.

:TEXT_CMDHELP_DEL#2
Bri�e jedan ili vi�e fajlova.

  DEL [drajv:][putanja]fajl [/P] [/V]
  ERASE [drajv:][putanja]fajl [/P] [/V]

  [drajv:][putanja]fajl   Navodi fajl(ove) za brisanje. Navedite vi�e
                          fajlova koriste�i se d�oker znacima.
  /P    Poziva upit za potvrdu pre brisanja svakog fajla pojedina�no.
  /V    Prikazuje sve izbrisane fajlove.
.

:TEXT_CMDHELP_DIR#4
Prikazuje listu fajlova i pod-direktorijuma u direktorijumu.

  DIR [drajv:][putanja][fajl] [/P] [/W] [/A[[:]atributi]]
       [/O[[:]na�in]] [/S] [/B] [/L] [/[Y|4]]

  [drajv:][putanja][fajl]
           Navodi drajv, direktorijum, i/ili fajlove za listanje.
            (Mo�e biti slo�ena ili vi�estruka specififikacija fajlova)
  /A       Prikazuje fajlove sa navedenim atributima. (Podraz. /ADHSRA)
  atributi  D  Direktorijumi             R  Read-only (samo za �itanje)
            H  Hidden (sakriveni)        A  Spremni za arhiviranje
            S  Sistemski fajlovi         -  Prefiks negacije
  /O       Prikazuje fajlove sortirane na navedeni na�in. (Podraz. /ONG)
  na�in     N  Po nazivu                 S  Po veli�ini
            E  Po ekstenziji             D  Po datumu i vremenu
            G  Grupi�e direktorijume     -  Prefiks inverznog redosleda
            U  Nesortirano
  /P       Pauzira posle svakog punog ekrana informacija.
  /W       Koristi format �irokog listanja.
  /S       Prikazuje fajlove u navedenom diru i svim pod-direktorijumima.
  /B       Koristi ogoljeni format (bez zaglavlja i statistike).
  /L       Koristi mala slova.
  /Y,/4    Prikazuje godinu sa 4 cifre.

 Prekida�i mogu biti navedeni u DIRCMD promenljivi okru�enja. Poni�tite
 ve� navedene prekida�e postavljanjem prefiksa "-". Na primer: /-W.
.

:TEXT_CMDHELP_DOSKEY#1
DOSKEY je omogu�en interno u komandnoj ljusci.

 �����������������������Ŀ
 � Taster     � Funkcija �
 ��������������������������������������������������������������������Ŀ
 � Gore/Dole  � Poziva istoriju.                                      �
 � Levo/Desno � Navigacija kroz komandnu liniju i/ili dopunjavanje    �
 �            � trenutne linije prethodno pozvanom komandom.          �
 � Home/End   � Po�etak/kraj linije.                                  �
 � Insert     � Promena izme�u moda umetanja i pisanja preko.         �
 � Tab        � Dopunjavanje trenutne re�i kao naziv fajla. Dvostruki �
 �            � pritisak �e prikazati sve odgovaraju�e fajlove.       �
 ����������������������������������������������������������������������

 Komanda HISTORY �e prikazati sadr�aj bafera istorije.
.

:TEXT_CMDHELP_ORIGINAL_DOSKEY#1
NA %1 %9
.

:TEXT_CMDHELP_ECHO
Prikazuje poruke, ili postavlja-uklanja odjek komandi.

  ECHO [ON | OFF]
  ECHO [poruka]
  ECHO.

 Pokrenite ECHO bez parametara za prikaz trenutne postavke.
 "ECHO." �e ispisati praznu liniju na ekran.
.

:TEXT_CMDHELP_EXIT
Napu�ta trenutno aktivnu komandnu ljusku.

  EXIT
.

:TEXT_CMDHELP_FOR
Pokre�e navedenu komandu za svaki fajl iz navedenog skupa fajlova.

  FOR %varijabla IN (skup) DO komanda [parametri]

  %varijabla    Navodi zamenljivi parametar.
  (skup)        Navodi skup od jednog ili vi�e fajlova. D�oker znaci su
                dozvoljeni.
  komanda       Navodi komandu za izvr�enje nad svakim fajlom iz skupa.
  parametri     Navodi parametre navedene komande.

 U batch programima, koristite %%varijabla umesto %varijabla.
.

:TEXT_CMDHELP_GOTO
Upu�uje komandnu ljusku na liniju odre�enu labelom u batch programu.

  GOTO labela

  labela   Navodi labelu definisanu u trenutnom batch programu.

 Labelu pi�ete samu na liniji, po�inju�i dvota�kom.
.

:TEXT_CMDHELP_HISTORY#1
Prikazuje trenutni sadr�aj bafera istorije.

  HISTORY
.

:TEXT_CMDHELP_IF
Vr�i uslovno procesiranje u batch programima.

  IF [NOT] ERRORLEVEL broj komanda
  IF [NOT] string1==string2 komanda
  IF [NOT] EXIST fajl komanda

  NOT               Navodi da komandna ljuska treba da izvr�i komandu samo
                    ukoliko je uslov neta�an. Negacija uslova.
  ERRORLEVEL broj   Vra�a TA�NO ukoliko je prethodno pokrenuti program
                    vratio izlazni kod jednak ili ve�i od navedenog broja.
  komanda           Navodi komandu koja se treba izvr�iti ukoliko je uslov
                    zadovoljen.
  string1==string2  Vra�a TA�NO ukoliko su nizovi znakova jednaki.
  EXIST fajl        Vra�a TA�NO ukoliko fajl sa navedenim imenom postoji.
.

:TEXT_CMDHELP_LH
U�itava program u gornje memorijsko podru�je.

  LOADHIGH [drajv:][putanja]fajl [parametri]
  LOADHIGH [/L:podru�je1[,min_vel1][;podru�je2[,min_vel2]...] [/S]]
           [drajv:][putanja]fajl [parametri]

  /L:podru�je1[,min_vel1][;podru�je2[,min_vel2]...
             Navodi podru�je(a) memorije u koje treba u�itati program.
             - podru�je1 navodi broj prvog memorijskog podru�ja.
             - min_vel1 navodi minimalnu veli�inu, ako postoji, za podru�je1.
             - podru�je2 navodi broj drugog memorijskog podru�ja.
             - min_vel2 navodi minimalnu veli�inu, ako postoji, za podru�je2.
             Mo�ete navesti koliko god �elite memorijskih podru�ja.

  /S         Smanjuje UMB do svoje minimalne veli�ine dok se program u�itava.

  [drajv:][putanja]fajl
             Navodi lokaciju i naziv programa koji treba u�itati.

  parametri  Parametri programa koji se u�itava.
.

:TEXT_CMDHELP_LOADFIX
U�itava program iznad prvih 64K memorije i pokre�e program.

  LOADFIX [drajv:][putanja]fajl

 Upotrebite LOADFIX za u�itavanje programa ukoliko ste dobili poruku
 "Packed file corrupt" pri poku�aju u�itavanja u nisku memoriju.
.

:TEXT_CMDHELP_MD
Kreira novi direktorijum.

  MKDIR [drajv:]putanja
  MD [drajv:]putanja
.

:TEXT_CMDHELP_PATH
Prikazuje ili postavlja putanju za tra�enje izvr�nih fajlova.

  PATH [[drajv:]putanja[;...]]
  PATH ;

 Pokrenite "PATH ;" za brisanje svih putanja i usmerenje komandne
 ljuske na tra�enje izvr�nih fajlova samo u trenutnom direktorijumu.
 Pokrenite PATH bez parametara za prikaz trenutne vrednosti.
.

:TEXT_CMDHELP_PAUSE
Zaustavlja procesiranje batch programa i prikazuje poruku:
"Pritisnite bilo koji taster za nastavak..." ili navedenu poruku.

  PAUSE [poruka]
.

:TEXT_CMDHELP_PROMPT
Menja odziv komandnog prompta.

  PROMPT [tekst]

  tekst    Navodi novi komandni prompt

 Prompt mo�e biti sastavljen od znakova i slede�ih specijalnih kodova:

      $Q   = (znak za jednako)
      $$   $ (znak za dolar)
      $T   Trenutno vreme
      $D   Trenutni datum
      $P   Trenutni drajv i putanja
      $V   Verzija komandne ljuske
      $N   Trenutni drajv
      $G   > (znak za ve�e-od)
      $L   < (znak za manje-od)
      $B   | (znak za cev)
      $H   Backspace (bri�e prethodni znak)
      $E   Escape kod (ASCII kod 27)
      $_   CR i LF kodovi (nova linija)

 Pokrenite PROMPT bez parametara za reset prompta na standardnu postavku.
.

:TEXT_CMDHELP_PUSHD
Gura trenutni direktorijum na vrh steka direktorijuma, uz opciju da promeni
trenutni radni direktorijum.

  PUSHD [[drajv:]putanja]

  Gde je [drajv:]putanja nova putanja koju �elite.
.

:TEXT_CMDHELP_POPD
Skida direktorijum sa vrha steka direktorijuma, i postavlja ga za radni.

  POPD [*]

  Opcija '*' bri�e sadr�aj steka direktorijuma.
.

:TEXT_CMDHELP_DIRS
Prikazuje sadr�aj steka direktorijuma.

  DIRS
.

:TEXT_CMDHELP_RD
Uklanja (bri�e) direktorijum.

  RMDIR [drajv:]putanja
  RD [drajv:]putanja
.

:TEXT_CMDHELP_REM
Obele�ava komentar (napomenu) u batch fajlu ili CONFIG.SYSu.

  REM [komentar]
.

:TEXT_CMDHELP_REN
Menja naziv fajlu ili direktorijumu.

  RENAME [drajv:][putanja][dir1 | fajl1] [dir2 | fajl2]
  REN [drajv:][putanja][dir1 | fajl1] [dir2 | fajl2]

 D�oker znaci nisu dozvoljeni.

 Primetite da ne mo�ete navesti novi drajv ili putanju za odredi�te.
 U tu svrhu upotrebite eksternu komandu MOVE.
.

:TEXT_CMDHELP_SET#1
Prikazuje, postavlja, ili uklanja promenljive okru�enja.

  SET [/C] [/P] [promenljiva=[string]]

  promenljiva  Navodi ime promenljive okru�enja.
  string       Navodi niz znakova za dodelu promenljivoj. Ukoliko string
               nije naveden, promenljiva se uklanja iz okru�enja.

 Pokrenite SET bez parametara za prikaz trenutnih promenljivi okru�enja.

  /C    Prisiljava zadr�avanje navedene veli�ine slova promenljive.
  /P    Poziva upit sa stringom i dodeljuje korisni�ki unos promenljivoj.

 Promenljiva se standardno menja u velika slova, ukoliko se ve� ne
 nalazi u okru�enju, ina�e se navedena veli�ina slova zadr�ava.
.

:TEXT_CMDHELP_SHIFT#1
Menja poziciju zamenljivih parametara u batch fajlu.

  SHIFT [DOWN]

 DOWN pomera prozor argumenata prema po�etku (%0). U ostalim slu�ajevima
 pomera isti prema kraju.
.

:TEXT_CMDHELP_TIME#1
Prikazuje ili postavlja sistemsko vreme.

  TIME [/T] [vreme]

  /T odvra�a TIME od interaktivnosti.

 Pokrenite TIME bez parametara za prikaz trenutne postavke vremena i
 upit za novu. Pritisnite ENTER da sa�uvate isto vreme.
.

:TEXT_CMDHELP_TRUENAME
Prikazuje punu putanju trenutne ili navedene putanje.

  TRUENAME [[drajv:][putanja][fajl]]
.

:TEXT_CMDHELP_TYPE
Prikazuje sadr�aj tekstualnih fajlova.

  TYPE [drajv:][putanja]fajl
.

:TEXT_CMDHELP_VER
Prikazuje verziju komandne ljuske i ostale informacije.

  VER [/R] [/W] [/D] [/C]

  /R         Verzija jezgra i ostale informacije.
  /W         Garancija komandne ljuske.
  /D         Informacije o pravilima redistribuiranja.
  /C         Imena saradnika.
.

:TEXT_CMDHELP_VERIFY
Postavlja ili uklanja dodatne provere korektnosti upisa fajlova na disk.

  VERIFY [ON | OFF]

 Pokrenite VERIFY bez parametara za prikaz trenutne postavke.
.

:TEXT_CMDHELP_FDDEBUG
NA
.

:TEXT_CMDHELP_VOL
Prikazuje labelu volumena i serijski broj, ukoliko isti postoje.

  VOL [drajv:]
.

:TEXT_CMDHELP_QUESTION#1
Prikazuje listu komandi i osobina dostupnih u ljusci.

  ?
  ?komanda [argument]

 Prva varijanta prikazuje dostupne komande i osobine ljuske.
 Druga varijanta �e izneti upit da li se navedena komanda treba
 izvr�iti, kao da je mod pra�enja aktivan.
.

:TEXT_CMDHELP_WHICH
Tra�i i prikazuje izvr�ne fajlove za svaku navedenu komandu.

  WHICH {komanda}
.

:TEXT_CMDHELP_MEMORY#1
NA
.

:TEXT_ERROR_COPY_PLUS_DESTINATION#1
Odredi�te COPY ne sme sadr�ati plus ('+') znakove.
.

:TEXT_DELETE_FILE#1
Brisanje fajla "%s".
.
