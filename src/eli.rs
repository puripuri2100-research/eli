/// 法令等の公開先
pub enum Published {
  /// URIがある場合
  Uri(String),
  /// URIが存在しない場合の文字情報
  Other(String),
  /// 非公開
  Private,
  /// 不明
  NoInformation,
}

/// European Legislation Identifier(ELI)を実装するトレイト
pub trait Eli {
  fn eli_uri(&self) -> String;
  fn published(&self) -> Published;
}

/// ELIで使用されるオントロジー
/// 定義となるRDFファイル: <http://data.europa.eu/eli/ontology>
/// 作成時(2025-10-21)ではバージョン1.5
pub enum EliOntology {
  /// 被参照を表す(<http://data.europa.eu/eli/ontology#amended_by>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Inverse of "amends";. Indicates a work that introduced legal changes in this resource. For modifications that don’t have a legal impact, use eli:corrected_by.
  AmendedBy,
  /// 参照を表す(<http://data.europa.eu/eli/ontology#amends>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  ///
  /// > Indicates that this work introduces legal changes in another resource. For modifications that don’t have a legal impact, use eli:corrects.
  Ammends,
  /// 他の法令から準拠されていることを表す(<http://data.europa.eu/eli/ontology#applied_by>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Inverse of "applies".
  /// > Note that this property is expressed on a legal resource, not on one of its language-specific legal expression.
  AppliedBy,
  /// 他の法令に準拠していることを表す(<http://data.europa.eu/eli/ontology#applies>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Indicates that this legislation (or part of a legislation) somehow conforms with another legislation. This is an informative link, and it has no legal value. For legally-binding links of transposition, use the property transposes. This can be used for example :
  /// > - when a pre-existing law already conforms to a recent european directive (in that case it does not &quot;transposes&quot; it strictly speaking);
  /// > - when non-EU member states make sure their legislation is conformant with EU law without strictly speaking transposing it;
  /// > - when a legislation from a local authority conforms with a national legislation;
  /// >
  /// > Note that this should point to a LegalResource, not to a language-specific expression.
  Applies,
  /// 他の法令から付託されていることを表す(<http://data.europa.eu/eli/ontology#based_on>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Inverse of &quot;basis_for&quot;. Indicates that thiswork is empowered by another one, typically a constitution, a treaty or an enabling act.
  BasedOn,
  /// 他の法令に対して付託していることを表す(<http://data.europa.eu/eli/ontology#basis_for>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Indicates that this work or expression empowers another . Typically primary legislation is the basis for secondary legislation.
  BasisFor,
  /// 他の法令によって改正・廃止などをされたことを表す(<http://data.europa.eu/eli/ontology#changed_by>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Inverse of « changes ». Indicates that this work or expression is being legally changed by another. This encompasses the notions of amendment, replacement, repeal, or other types of change.
  ChangedBy,
  /// 他の法令を改正．廃止など変更を加えたことを表す(<http://data.europa.eu/eli/ontology#changes>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Indicates that this work or expression legally changes another. This encompasses the notions of amendment, replacement, repeal, or other types of change. This may be a direct change (textual or non-textual amendment) or a consequential or indirect change. Note, the property is to be used to express the existence of a change relationship between two acts rather than the existence of a consolidated version of the text that shows the result of the change. For consolidation relationships, use the &quot;consolidates&quot; and &quot;consolidated_by&quot; properties.
  Changes,
  /// 「何らかの文献が法令によって引用されている」ことを表す(<http://data.europa.eu/eli/ontology#cited_by>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Inverse of &quot;cites&quot;.
  /// > Note that the intended meaning of this link is to indicate that &quot;something is cited by a legislation&quot; and not that &quot;this legislation is cited by something&quot;.
  CitedBy,
  /// URIで識別されている判例法を参照しているときに使用する(<http://data.europa.eu/eli/ontology#cited_by_case_law>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Indicates that this LegalResource or LegalExpression is being cited in a case law, identified by a suitable URI. If the case law cannot be identified by a suitable URI, the property &quot;eli:cited_by_case_law_reference&quot; can be used with a textual reference to the case law.
  /// >
  /// > The actual citation link is expressed from the case law to the legislation, but legal portals may use the link from a legislation to a case law to e.g. refer to representative case laws about a legislation.
  CitedByCaseLaw,
  /// 法令本文中の引用を表す: (<http://data.europa.eu/eli/ontology#cites>)
  ///
  /// <http://data.europa.eu/eli/ontology>における説明:
  /// > Citation in the text of the legislation. This may be at the legal resource or legal expression level, as required by the implementation context. This includes verbatim citation and citations in referrals.
  Cites,
  /// > Inverse of &quot;commences&quot;. Indicates that this legal resource was set in force by another legal resource.
  /// > Situations where a resource enters into force because of more than one resource are explicitely allowed.
  CommencedBy,
  /// > Indicates that this legal resource sets another legal resource into force.
  /// > Note the the date of entry into force of the other resource should be modified accordingly.
  /// > Note also that it is not possible to indicate when the entry into force should happen.
  Commences,
  /// > Inverse of &quot;consolidates&quot;. Indicates that this legal resource or expression is taken into account in a consolidated text (which is usually the product of an editorial process that revises the legislation).
  ConsolidatedBy,
  /// > Indicates that this consolidated legal resource or expression (which is usually the product of an editorial process that revises the legislation) takes into account another one. This property should be used multiple times to refer to both the original version or the previous consolidated version, and to the legislations making the change.
  Consolidates,
  /// > Inverse of &quot;corrects&quot;. Indicates a resource that introduces textual modifications (like correction of spelling mistakes) with no legal change in this work, expression or manifestation; typically corrigenda in EU legislation. For modifications that have a legal impact, use eli:amended_by.
  CorrectedBy,
  /// > Indicates that this work introduces textual modifications (like correction of spelling mistakes) with no legal change in another resource, expression or manifestation; typically corrigenda in EU legislation. For modifications that have a legal impact, use eli:amends.
  Corrects,
  /// > A person or organization that countersigned the legislation. Depending on the legal context, a countersignature can indicate that the signed authority undertakes to assume responsibility for texts emanating from a person who is inviolable and irresponsible, (for example a King, Grand Duc or President), or that the authority is in charge of the implementation of the text.
  CountersignedBy,
  /// > Relates a manifestation to the expression that it embodies. Inverse of &quot;is_embodied_by&quot;.
  Embodies,
  /// > Indicates that this LegalResource ensures the implementation of another LegalResource. This implies a legal meaning (contrary to eli:applies).
  /// >
  /// > This can cover links from national legislation to EU legislation, or links from regional to national legislation. It can also cover links from EU implementing acts to EU legislative acts. In the case of implementation of EU legislation at national level, this covers links to EU regulations, decisions, etc. However the transpositions of EU Directives must be captured with eli:transposes.
  /// >
  /// > Links between national primary and secondary legislation must be captured by eli:based_on / eli:basis_for.
  EnsuresImplementationOf,
  /// > The format of the manifestation, expressed as a custom URI.
  /// > This field is intended to capture the format of the resource from an application or user perspective, as opposed to the &quot;media_type&quot; property that expresses its format from a technical point of view. This property allows to describe different XML schemas (Akoma N&apos;toso vs. proprietary), describe different types of PDFs (scanned PDF, generated-on-the-fly PDF, signed PDF, archival PDF) or describe the paper (printed) version of the legislation.
  /// > ELI includes a set of possible values for the most common use-cases and possible URIs values CAN also be taken from <http://www.iana.org/assignments/media-types> , or can de defined by the Member States.
  Format,
  /// > Indicates an annex to this this work or expression
  HasAnnex,
  /// > Inverse of is_derivative_of
  HasDerivative,
  /// > Indicates that this work conceptually includes another one. For the notion of physical inclusion, use eli:has_part.
  HasMember,
  /// > inverse of &quot;is_part_of&quot;
  HasPart,
  /// > Inverse of &quot;is_translation_of&quot;. Indicates that this expression has been translated into another derived expression. See the definition of &quot;is_translation_of&quot;.
  HasTranslation,
  /// > Indicates that the implementation of this LegalResource is ensured by another LegalResource. This implies a legal meaning (contrary to eli:applies). See the definition of eli:ensures_implementation_of.
  Implements,
  /// > A value indicating the legal force of a legal resource or a legal expression. A set of values is defined by ELI in the corresponding concept scheme. These values are :
  /// > - in force
  /// > - partially in force
  /// > - not in force
  InForce,
  /// > A subject for this work. The use of Eurovoc (<http://eurovoc.europa.eu>) is encouraged to select values for this property. Member states are encouraged to align local values to Eurovoc.
  IsAbout,
  /// > Indicates this work or expression is an annex of another one.
  IsAnnexOf,
  /// > Indicates that this resource is a new publication, in a different official journal, of another resource already published elsewhere, and cannot be considered to be the same resource (owl:sameAs cannot be used to avoid potential duplication of certain metadata, like the date of publication, or the publisher).
  /// >
  /// > Note that this is different from the exceptionnal cases of &quot;republication&quot;, where the same resource is actually republished in the same official journal a few days after its initial publication, in case of errors.
  IsAnotherPublicationOf,
  /// > A Work or Expression from which this one derive
  IsDerivativeOf,
  /// > Relates an expression to a manifestation of that expression. Inverse of &quot;embodies&quot;.
  IsEmbodiedBy,
  /// > Link to a concrete file URL.
  /// > Relates a manifestation to a single exemplar or instance of that manifestation.
  IsExemplifiedBy,
  /// > Indicates that this work is conceptually included in another one. In the case of a legislation, its successive temporal versions are conceptually members of a single « abstract » resource. For the notion of physical inclusion, use eli:is_part_of.
  IsMemberOf,
  /// > Indicates a work in which this one is physically included. Covers the case of text included in an Official Journal, or an article included in a text. For the notion of conceptual/temporal inclusion, use eli:is_member_of.
  IsPartOf,
  /// > Relates a work to an expression of this work in the form of a &quot;sequence of signs&quot; (typically alpha-numeric characters in a legal context). Inverse of &quot;realizes&quot;.
  IsRealizedBy,
  /// > Indicates a work or expression that refers to this entity.
  IsReferredToBy,
  /// > Indicates that this expression has been translated from another original expression; this can be used to distinguish original from derived expressions.
  /// >
  /// > Note that asserting this link does not have any implication on the legal value of the original and translated expressions : depending on the context, a translation can be as legally binding as the original version, or can be published for informative purposes only (e.g. a country translating some legal acts in English).
  /// >
  /// > The expressions linked with this property can be realisations of the same LegalResource, or different LegalResources. Multilingual legislations that do not need to distinguish between original and derived expressions of the same LegalResource (e.g. european legislation) will not use this property.
  IsTranslationOf,
  /// > The jurisdiction from which the legal resource originates.
  /// >
  /// > The place identifier can be taken from the Administrative Territorial Unit table published of the EU Publications Office at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/atu>. Member States don&apos;t have to recreate their own list of values.
  Jurisdiction,
  /// > The language of an expression.
  /// >
  /// > EU Publications Office provides a list of languages at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/language>. This list is large enough so that member states should not have to declare local values.
  /// >
  /// > Note that, if needed, a language can also be stated on a legal resource using the DublinCore &quot;language&quot; property.
  Language,
  /// > The legal value associated with a specific format of a resource. A set of values is defined by ELI in the corresponding concept scheme. These values are :
  /// > - unofficial : no particular or special standing;
  /// > - official : published by an organisation with the public task of making the information available (e.g. a consolidated version of a EU directive) ;
  /// > - authoritative : the publisher gives some special status to the publication (e.g. &quot;the Queens Printer&quot; version of an Act of Parliament, or the OJ version of a EU Directive);
  /// > - definitive : the text is conclusively what the law says, (e.g. the digitally signed version of an OJ).
  LegalValue,
  /// > A legal document giving official permission to do something with the resource (Definition from Dublin Core)
  License,
  /// > The file format of the manifestation.
  /// > This field is intended to capture the technical file format and will serve as a basis for content negotiation for the server to return the appropriate file based on the client preference.
  /// > Although not mandatory, this property is highly encouraged.
  /// > Possible URIs values MUST be taken from <http://www.iana.org/assignments/media-types> (e.g. <http://www.iana.org/assignments/media-types/application/xml>).
  /// > See also the &quot;format&quot; property.
  MediaType,
  /// > The person or organization that originally passed or made the law : typically parliament (for primary legislation) or government (for secondary legislation). This indicates the &quot;legal author&quot; of the law, as opposed to its physical author.
  /// >
  /// > This property can be used multiple times to indicate both the agent that authored the text, and/or the agent that signed the text, thus turning it into an actual legislation.
  /// >
  /// > The relationship between current and any former law making body should be represented in the description of the agent itself.
  /// >
  /// >Member states are encouraged to make their own list of Agents. EU Publications Office provides a list of corporate bodies at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/corporate-body>.
  PassedBy,
  /// > Reference to the Official Journal or other publication manifestation in which this format is published.
  /// > This property should be used when the value can be identified by a suitable URI; in the absence of such a URI, the property &quot;published_in&quot; should be used with a string value.
  PublishedInFormat,
  /// > An entity responsible for making the resource available (definition from Dublin Core).
  /// > This property should be used when the value can be identified by a suitable URI; in the absence of such a URI, the property &quot;publisher&quot; should be used with a string value.
  PublisherAgent,
  /// > Inverse of &quot;published_in_format&quot;. Note this property does not link a publisher with a resource, but rather a specific Format of a resource with a specific Format of another resource, indicating that the subject Format publishes the object Format.
  Publishes,
  /// > Relates an expression to a work realised through that expression. Inverse of &quot;is_realized_by&quot;.
  Realizes,
  /// > Any entity that this work or expression refers to ; typically references are made to other Works, but it can be also to legislative processes, persons, etc.
  /// >
  /// > Note that specific subproperties exist in ELI-DL to describe future legal impacts (\&quot;foresees_xxxx\&quot; links) and links between amendments and draft legislation.
  /// >
  /// > Example : an opinion refers to the specific version of the draft legislation on which it is based.
  RefersTo,
  /// > Indicates a somehow related other document, not necessarily a legal resource. Note that citation links should use the cites property.
  RelatedTo,
  /// > Refers to a place or an area associated with the resource. This covers the notions of jurisdiction, sovereignty, applicability or administrative area. The place identifier can be taken from the Administrative Territorial Unit table published of the EU Publications Office at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/atu>. Member States don&apos;t have to recreate their own list of values.
  /// >
  /// > The group notes the limitations of what can be said with a single property; member states can refine this notion by declaring specific sub properties.
  RelevantFor,
  /// > Inverse of &quot;repeals&quot;. Indicates that this legal resource or legal expression is being completely canceled, abrogated or replaced by another legal resource. If a resource is partially repealed by another, this link can be used at the corresponding subdivision level being completely repealed.
  RepealedBy,
  /// > Indicates that this legal resource or legal expression completely cancels, abrogates or replaces another. If a resource partially repeals another, this link can be used at the corresponding subdivision level being completely repealed.
  Repeals,
  /// > An individual, organisation or organisational unit that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
  /// > This property should be used when the value can be identified by a suitable URI; in the absence of such a URI, the property &quot;responsibility_of&quot; should be used with a string value.
  ResponsibilityOfAgent,
  /// > A person or organisation owning or managing rights over the resource (definition from Dublin Core).
  /// > This property should be used when the value can be identified by a suitable URI; in the absence of such a URI, the property &quot;rightsholder&quot; should be used with a string value.
  RightsholderAgent,
  /// > Inverse of &quot;transposes&quot;.
  /// >
  /// > Note that this property is expressed on a legal resource, not on one of its language-specific legal expression.
  TransposedBy,
  /// > Indicates that this legislation (or part of legislation) fulfills the objectives set by another legislation, by passing appropriate implementation measures. Typically, some legislations of European Union&apos;s member states or regions transpose European Directives. This indicates a legally binding link between the 2 legislations, at act or article level, from the original version of a national implementing measure to the legal resource Directive as published in the EU Official Journal. Can be used for transposition tables, once EU Publication Office has introduced ELI support down to the article level.
  /// >
  /// > The implementation of EU legislation at national level, involving links to EU regulations, decisions, etc. must be captured with eli:ensures_implementation_of.
  /// >
  /// > Note that this should point to the legal resource of the Directive itself, not to one of its language-specific legal expression.
  Transposes,
  /// > The type of a legal resource (e.g. &quot;Directive&quot;, &quot;Règlement grand ducal&quot;, &quot;law&quot;, &quot;règlement ministériel&quot;, &quot;draft proposition&quot;, &quot;Parliamentary act&quot;, etc.).
  /// >
  /// > Member states are encouraged to make their own list of values in the corresponding concept scheme. EU Publications Office provides a list of values for EU resource types at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/resource-type>
  TypeDocument,
  /// > The type of a document subdivision (e.g. &quot;Article&quot;, &quot;Paragraph&quot;, &quot;Section&quot;, etc.).
  /// > A subdivision can have only one type.
  /// >
  /// > ELI does not specify a list of possible values. Member states are encouraged to make their own list of values in the corresponding concept scheme. EU Publication Office provies a list of values for EU resource types at <https://op.europa.eu/en/web/eu-vocabularies/at-dataset/-/resource/dataset/subdivision>
  TypeSubdivision,
  /// > Schema describing the URI of an ELI instance. ELI uses URI template specifications (IETF RFC 6570). Schemes should be associated with member states and will be published in a registry.
  UriSchema,
  /// > A skos concept scheme, could be locally defined? Group proposal is to start with an initial ELI scheme, that might include concepts of &quot;Official Journal&quot; &quot;made&quot; &quot;consolidated&quot; &quot;proposed&quot; &quot;prospective&quot;
  Version,
  /// > The type of a work, taken from a controlled vocabulary.
  /// >
  /// > Member States need to define their own work type values.
  WorkType,
  /// > Indicates that this LegalResource or LegalExpression is being cited in a case law that cannot be identified by a suitable URI and that is indicated by textual reference. An ECLI (European Case Law Identifier) can be used here.
  /// > When the case law can be identified by a suitable URI, the property eli:cited_by_case_law should be used instead.
  CitedByCaseLawReference,
  /// > The date at which the legislation becomes applicable. This is distinct from the date of entry into force : a text may come in force today, and state it will become applicable in 3 months.
  /// >
  /// > The group notes that applicability dates can lead to complex situations, e.g. a text with different applicability dates for different jurisdictions; specific extensions to the model should be used for such situations.
  DateApplicability,
  /// > Date of adoption or signature (of the form yyyy-mm-dd)
  DateDocument,
  /// > The last date any part of the legislation is in force, if the date is known (can be seen as the end date of a dc:valid range for this resource).
  DateNoLongerInForce,
  /// > Date of publication of the official version of the legislation, in hard copy or online, depending on what the official publication is, and when it was published. Publication dates at the level of legal expressions can be separately asserted, using standard Dublin Core properties.
  DatePublication,
  /// > An account of the resource (definition from Dubin Core), e.g a summary.
  Description,
  /// > The first date any part of the legal resource or legal expression came into force (can be seen as the start date of a dc:valid range for this resource)
  FirstDateEntryInForce,
  /// > The unique identifier used in a local reference system to maintain backwards compatibility. For examples the CELEX at EU level, or the NOR in France.
  IdLocal,
  /// > An identifier or other disambiguating feature for a work or expression. This can be the number of a legislation, the number of an article, or the issue number of an official journal.
  Number,
  /// > Reference to the Official Journal or other publication manifestation in which this format is published.
  /// > This property should be used when the value cannot be identified by a suitable URI; if a URI is available, the property &quot;published_in_format&quot; should be used.
  PublishedIn,
  /// > An entity responsible for making the resource available (definition from Dublin Core).
  /// > This property should be used when the value cannot be identified by a suitable URI; if a URI is available, the property &quot;publisher_agent&quot; should be used.
  Publisher,
  /// > An individual, organisation or organisational unit that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
  /// > This property should be used when the value cannot be identified by a suitable URI; if a URI is available, the property &quot;responsibility_of_agent&quot; should be used.
  ResponsibilityOf,
  /// > Information about rights held in and over the resource (definition from Dublin Core). For example, that property can be used to provide a link to a page that describes the licensing terms.
  Rights,
  /// > A person or organisation owning or managing rights over the resource (definition from Dublin Core).
  /// > This property should be used when the value cannot be identified by a suitable URI; if a URI is available, the property &quot;rightsholder_agent&quot; should be used.
  Rightscholder,
  /// > The title, or name, of an expression.
  /// >
  /// > Note that, if needed, a title can also be stated on a legal resource using the Dublin Core &quot;title&quot; property.
  Title,
  /// > An alternative title of the expression (if any).
  /// >
  /// > Note that, if needed, an alternative title can also be stated on a work using the Dublin Core &quot;alternative&quot; property.
  TitleAlternative,
  /// > Established short title of the expression (if any)
  TitleShort,
  /// > The point-in-time at which the provided description of the legislation is valid.
  VersionDate,
}

impl EliOntology {
  /// ELI Ontologyで定義されているURIにする
  pub fn uri(&self) -> String {
    match self {
      Self::AmendedBy => String::from("http://data.europa.eu/eli/ontology#amended_by"),
      Self::Ammends => String::from("http://data.europa.eu/eli/ontology#amends"),
      Self::AppliedBy => String::from("http://data.europa.eu/eli/ontology#applied_by"),
      Self::Applies => String::from("http://data.europa.eu/eli/ontology#applies"),
      Self::BasedOn => String::from("http://data.europa.eu/eli/ontology#based_on"),
      Self::BasisFor => String::from("http://data.europa.eu/eli/ontology#basis_for"),
      Self::ChangedBy => String::from("http://data.europa.eu/eli/ontology#changed_by"),
      Self::Changes => String::from("http://data.europa.eu/eli/ontology#changes"),
      Self::CitedBy => String::from("http://data.europa.eu/eli/ontology#cited_by"),
      Self::CitedByCaseLaw => String::from("http://data.europa.eu/eli/ontology#cited_by_case_law"),
      Self::Cites => String::from("http://data.europa.eu/eli/ontology#cites"),
      Self::CommencedBy => String::from("http://data.europa.eu/eli/ontology#Commenced_by"),
      Self::Commences => String::from("http://data.europa.eu/eli/ontology#Commences"),
      Self::ConsolidatedBy => String::from("http://data.europa.eu/eli/ontology#consolidated_by"),
      Self::Consolidates => String::from("http://data.europa.eu/eli/ontology#consolidates"),
      Self::CorrectedBy => String::from("http://data.europa.eu/eli/ontology#corrected_by"),
      Self::Corrects => String::from("http://data.europa.eu/eli/ontology#correccts"),
      Self::CountersignedBy => String::from("http://data.europa.eu/eli/ontology#countersigned_by"),
      Self::Embodies => String::from("http://data.europa.eu/eli/ontology#embodies"),
      Self::EnsuresImplementationOf => {
        String::from("http://data.europa.eu/eli/ontology#ensures_implementation_of")
      }
      Self::Format => String::from("http://data.europa.eu/eli/ontology#format"),
      Self::HasAnnex => String::from("http://data.europa.eu/eli/ontology#has_annex"),
      Self::HasDerivative => String::from("http://data.europa.eu/eli/ontology#has_derivative"),
      Self::HasMember => String::from("http://data.europa.eu/eli/ontology#has_member"),
      Self::HasPart => String::from("http://data.europa.eu/eli/ontology#has_part"),
      Self::HasTranslation => String::from("http://data.europa.eu/eli/ontology#has_translation"),
      Self::Implements => String::from("http://data.europa.eu/eli/ontology#implements"),
      Self::InForce => String::from("http://data.europa.eu/eli/ontology#in_force"),
      Self::IsAbout => String::from("http://data.europa.eu/eli/ontology#is_about"),
      Self::IsAnnexOf => String::from("http://data.europa.eu/eli/ontology#is_annex_of"),
      Self::IsAnotherPublicationOf => {
        String::from("http://data.europa.eu/eli/ontology#is_another_publication_of")
      }
      Self::IsDerivativeOf => String::from("http://data.europa.eu/eli/ontology#is_derivative_of"),
      Self::IsEmbodiedBy => String::from("http://data.europa.eu/eli/ontology#is_embodied_by"),
      Self::IsExemplifiedBy => String::from("http://data.europa.eu/eli/ontology#is_exemplified_by"),
      Self::IsMemberOf => String::from("http://data.europa.eu/eli/ontology#is_member_of"),
      Self::IsPartOf => String::from("http://data.europa.eu/eli/ontology#is_part_of"),
      Self::IsRealizedBy => String::from("http://data.europa.eu/eli/ontology#is_realized_by"),
      Self::IsReferredToBy => String::from("http://data.europa.eu/eli/ontology#is_referred_to_by"),
      Self::IsTranslationOf => String::from("http://data.europa.eu/eli/ontology#is_translation_of"),
      Self::Jurisdiction => String::from("http://data.europa.eu/eli/ontology#jurisdiction"),
      Self::Language => String::from("http://data.europa.eu/eli/ontology#lanuguage"),
      Self::LegalValue => String::from("http://data.europa.eu/eli/ontology#legal_value"),
      Self::License => String::from("http://data.europa.eu/eli/ontology#license"),
      Self::MediaType => String::from("http://data.europa.eu/eli/ontology#media_type"),
      Self::PassedBy => String::from("http://data.europa.eu/eli/ontology#passed_by"),
      Self::PublishedInFormat => {
        String::from("http://data.europa.eu/eli/ontology#published_in_format")
      }
      Self::PublisherAgent => String::from("http://data.europa.eu/eli/ontology#publisher_agent"),
      Self::Publishes => String::from("http://data.europa.eu/eli/ontology#publishes"),
      Self::Realizes => String::from("http://data.europa.eu/eli/ontology#realizes"),
      Self::RefersTo => String::from("http://data.europa.eu/eli/ontology#refers_to"),
      Self::RelatedTo => String::from("http://data.europa.eu/eli/ontology#related_to"),
      Self::RelevantFor => String::from("http://data.europa.eu/eli/ontology#relevant_for"),
      Self::RepealedBy => String::from("http://data.europa.eu/eli/ontology#repealed_by"),
      Self::Repeals => String::from("http://data.europa.eu/eli/ontology#repeals"),
      Self::ResponsibilityOfAgent => {
        String::from("http://data.europa.eu/eli/ontology#responsibility_of_agent")
      }
      Self::RightsholderAgent => {
        String::from("http://data.europa.eu/eli/ontology#Rightsholder_agent")
      }
      Self::TransposedBy => String::from("http://data.europa.eu/eli/ontology#transposed_by"),
      Self::Transposes => String::from("http://data.europa.eu/eli/ontology#transposes"),
      Self::TypeDocument => String::from("http://data.europa.eu/eli/ontology#type_document"),
      Self::TypeSubdivision => String::from("http://data.europa.eu/eli/ontology#type_subdivision"),
      Self::UriSchema => String::from("http://data.europa.eu/eli/ontology#uri_schema"),
      Self::Version => String::from("http://data.europa.eu/eli/ontology#version"),
      Self::WorkType => String::from("http://data.europa.eu/eli/ontology#work_type"),
      Self::CitedByCaseLawReference => {
        String::from("http://data.europa.eu/eli/ontology#cited_by_case_law_reference")
      }
      Self::DateApplicability => {
        String::from("http://data.europa.eu/eli/ontology#date_applicability")
      }
      Self::DateDocument => String::from("http://data.europa.eu/eli/ontology#date_document"),
      Self::DateNoLongerInForce => {
        String::from("http://data.europa.eu/eli/ontology#date_no_longer_in_force")
      }
      Self::DatePublication => String::from("http://data.europa.eu/eli/ontology#date_publication"),
      Self::Description => String::from("http://data.europa.eu/eli/ontology#description"),
      Self::FirstDateEntryInForce => {
        String::from("http://data.europa.eu/eli/ontology#first_date_entry_in_force")
      }
      Self::IdLocal => String::from("http://data.europa.eu/eli/ontology#id_local"),
      Self::Number => String::from("http://data.europa.eu/eli/ontology#number"),
      Self::PublishedIn => String::from("http://data.europa.eu/eli/ontology#published_in"),
      Self::Publisher => String::from("http://data.europa.eu/eli/ontology#publisher"),
      Self::ResponsibilityOf => {
        String::from("http://data.europa.eu/eli/ontology#responsibility_of")
      }
      Self::Rights => String::from("http://data.europa.eu/eli/ontology#rights"),
      Self::Rightscholder => String::from("http://data.europa.eu/eli/ontology#rightsholder"),
      Self::Title => String::from("http://data.europa.eu/eli/ontology#title"),
      Self::TitleAlternative => {
        String::from("http://data.europa.eu/eli/ontology#title_alternative")
      }
      Self::TitleShort => String::from("http://data.europa.eu/eli/ontology#title_short"),
      Self::VersionDate => String::from("http://data.europa.eu/eli/ontology#version_date"),
    }
  }
}
