//! # Aphorism
//!
//! This module contains the papi's aphorisms

use rand::Rng;

pub struct Aphorism;

impl Aphorism {
    /// Get a random aphorism of the papi
    pub fn get_random() -> &'static str {
        let mut rng = rand::thread_rng();
        PAPI_APHORISM[rng.gen_range(0..PAPI_APHORISM.len())]
    }
}

const PAPI_APHORISM: &[&str] = &[
    "La parola più importante che puoi imparare a dire? NO!",
    "Per fare la grana devi svegliarti presto.",
    "Lazzarone, alzati e lucra!",
    "Il vero complotto è stato fatto al danno dei giovani.",
    "Rispondi male ai tuoi clienti e fai montagne di soldi.",
    "Non fate i barboni per due lire.",
    "Perfezionista? Allora sei povero in canna.",
    r#"Chi ti dice "Sì, ma con calma che sei giovane" quando hai 18 anni... è lo stesso che a 30 anni ti diceva "eh, ma dovevi pensarci prima"."#,
    "Io potrei tranquillamente non fare niente, ma non voglio.",
    "Smetto di lavorare e vivo di rendita.",
    "Vuoi diventare ricco? Non hai tutto il tempo del mondo.",
    "Dovete avere 10 mesi di stile di vita in banca.",
    "Se dovete imparare come fare un'azienda, non potete più andare al pub con i vostri amici sfigati a bere la birra.",
    "Moltissimi \"imprenditori\" non mostrano i prezzi per paura che la concorrenza li copi. Ma stiamo scherzando?",
    "Quando ti diranno che parli sempre della stessa cosa vuol dire che stai andando bene.",
    "A.A.A. Cercasi mastino aziendale",
    "All'inizio dovete convertire chi è già in target. Non potete andare su un target più freddo",
    "Meno di 7 ore e addio profitti stellare con il tuo infobusiness",
    "Dovete avere una reale competenza. Se non avete una reale competenza, siete nei casini.",
    "Non potete copiare gli altri. Gli altri brancolano nel buio. Non lo hanno mai saputo e non lo sapranno mai",
    "Sto per farti esplodere il cervello!",
    "Una persona che guadagna 30 mila euro al mese e ne spende 30 mila è più povera di uno che ne guadagna 3 mile e se ne mette via 2 mila.",
    "Tutto il tuo marketing, la comunicazione e il tuo ecosistema deve essere fatto per vendere a prezzi alti.",
    "Io ho risultati, loro NO.",
    "L'essere grigio ti farà andare d'accordo con tutti, ma non ti posizionerà mai come un leader.",
    "Per avere dei soldi dovete vendere qualcosa a qualcuno che valuti quella cosa più di quello che sta pagando.",
    "Verità messianiche: l'impiegato non riesce a capire perché viene pagato.",
    "A volte anche solo avere le palle di chiedere più soldi funziona a priori.",
    "Il conto risparmio non si tocca. Ci morirete con quel conto. Più cresce e più sarete potenti.",
    "Ce l'ho duro come la coda del canguro!",
    "Se sei povero non puoi uscire fuori a mangiare. Le vacanze ti sono precluse.",
    "Marketing online è: come organizziamo un ecosistema che faccia uso di diversi strumenti per vendere in modo organizzato prodotti e servizi ad un mercato di riferimento.",
    "Perché i guru sulle Lamborghini con le fighe tirano così tanto? Perché diciamo che ad un uomo non gli farebbe proprio schifo come situazione.",
    "Dei tuoi titoli ufficiali non frega niente a nessuno. La gente non compra da te perché hai 10 lauree.",
    "Se tu dici: \"perdi peso così sei più in forma\" fallisci. La gente vuole dimagrire per scopare di più.",
    "Sei anche tu bloccato dalla paralisi da analisi e dalle troppe informazioni?",
    "La gente non vuole acquisire competenze. Tu gli devi vendere il risultato che otterranno dopo aver acquisito le competenze che gli stai vendendo.",
    "Crea la community, i likes e gli iscritti... e poi non si capisce come questa cosa vada monetizzata.",
    "Molte persone pensano di diventare ricche producendo tanti soldi quando invece la ricchezza è come gestiamo i soldi.",
    "Ho sempre invidiato chi andava dalle scuole internazionali e sapeva 5 lingue diverse. Questi sono i vantaggi dei soldi.",
    "Dovete arrivare ad avere almeno 10 mesi di stile di vita in banca. Ed è solo l'inizio.",
    "Questa volta lucro IO!",
    "Se avete degli obiettivi, non potete uscire con quelli che dicono YOLO.",
    "Nelle case dei ricchi al 99% la televisione non c'è.",
    "Il tuo mutuo o l'affitto non può costare più del 25% di quello che guadagni.",
    "Chi è uno sfigato? Uno che non ha obiettivi e si ritrova a 50 anni a dire \"boh\".",
    "Ci siamo, è il momento di lucrare!",
    "Per farvi pagare di più dovete rendervi insostitutibili.",
    "Tutti gli atleti e gli attori che finiscono in bancarotta è perché fanno un mucchio di soldi e oltre a sputtanarseli li danno in gestioni a manager che li derubano.",
    "I vostri amici col SUV al 99% stanno facendo finta e quindi sono bugiardi. Perché frequentate dei bugiardi?",
    "Tutte le persone ricche e di successo non guardano la TV.",
    "Quando si vende ai ricchi si hanno molti meno problemi.",
    "Non si arriva a questo essendo grigi. Bisogna essere polarizzanti.",
    "Dalla mediocrità si esce in modo fanatico, verticale ed in aggressività.",
    "Il pirla che ha cominciato con quel prezzo da fame, ha fatto praticamente morire di fame tutti gli altri che l'hanno copiato.",
    "Vendere ai poveri è un pessimo modello di business.",
    "Basta poco per giustificare l'aumento dei prezzi. Prendete Nusret, dove paghi la carne centinaia di dollari solo perché lui o chi per lui ti mette il sale con un movimento scenico.",
    "Se avete capito l'effetto composto pensate che avete 16 anni e mettete 200$ al mese nella borsa. È bellissimo.",
    "La normalità dovrebbe essere andare in tutti i ristoranti che volete senza guardare il prezzo.",
    "Non esiste nessuna azienda che prosperi con margini bassi.",
    "Io ho amici decamilionari, a cui avrei regalato i miei corsi, che li comprano senza dirmelo, perché sono persone con un'altra mentalità, non da poveri.",
    "Se io faccio pagare una pizza 15€ e me la pagano... è tutto a posto.",
    "Uno degli errori è guardare la concorrenza e copiargli il prezzo o addirittura abbassarlo.",
    "Non è più difficile vendere a prezzi alti. Lo è se provate a vendere a prezzi alti ai poveri.",
    "Non è vero che è più facile vendere low ticket invece che high ticket.",
    "Arrivati ad un certo livello ragazzi non è più il marketing a fare la differenza...",
    "Ci sono persone di successo che NON si alzano alle 5-6-7 di mattino.",
    "Urge privatino!",
    "Chi è povero non capisce i ricchi, sono come alieni.",
    "Le persone sono pericolose, voi dovete essere terrorizzati dalle persone",
    "La vostra vita è una merda, perché siete circondati dalle persone tossiche. Dovete isolarvi.",
    "Le notifiche sono il MALE assoluto.",
    "Se siete delle persone ambiziose e volete di più dalla vita dovete ISOLARVI.",
    r#"Nota del papi:
Signori davvero...
Chiunque abbia un infobusiness, venda videocorsi, consulenza o servizi done-for-you...
NON può fare a meno di queste informazioni.
E lo confermano i risultati e le testimonianze degli studenti che grazie alle stesse sono stati in grado di RADDOPPIARE, TRIPLICARE o addirittura DECUPLICARE i propri profitti netti nel giro di pochi mesi.
Inoltre, lo dico chiaro e tondo...

CHIUNQUE non abbia accesso a queste informazioni nel 2022 semplicemente ha un HANDICAP...
Ed è destinato a farsi ECLISSARE dai propri competitor."#,
    "Le persone normali non hanno obiettivi, non avendo obiettivi non possono sprecare il tempo.",
    "Le persone vi stanno rovinando la vita.",
    "La povertà è una malattia mentale",
    "È giusto lucrare su ogni opportunità.",
    "Noleggiare i costi, acquisire i ricavi.",
    "Ci sarà il link da qualche parte...",
    "Dove c'è aggregazione c'è mediocrità.",
    "Povero ma dignitoso, sta minchia.",
    "Il papi sono IO.",
    "Apprpinquatevi signori.",
    "Non vogliono solo 5 minuti del vostro tempo, questi vogliono vedervi fallire.",
    "Il sottobosco è il vero bosco.",
    "Fare marketing non è 'i facebook ADS'",
    "I soldi sono la libertà.",
    "Le persone mentono e rubane se ne hanno la possibilitò.",
    "La media è il popolume che il gira il mondo nutrendosi di banane."
];

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_get_papi_aphorism() {
        assert!(!Aphorism::get_random().is_empty());
    }
}
