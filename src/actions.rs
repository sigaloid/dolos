enum Actions {
	ActiveLanguages,
	AddPredictionDrafts,
	AllDynamicConfigs,
	AllPosts,
	AvailableAwards,
	AvatarCatalog,
	AwardingInfosByIds,
	AwardingTotalsForComment,
	AwardingTotalsForPost,
	BadgeCount,
	BlockAwarderByAwardingId,
	BlockedRedditors,
	BlockedUsers,
	CancelEconRecurringPayment,
	CancelPrediction,
	ChangePrediction,
	ChangePredictionVote,
	ChatTabBadge,
	ClaimAwardOffer,
	CoinPackOffers,
	CommentsPageAdPost,
	CreateAvatar,
	CreateAvatarShare,
	CreateChannelLink,
	CreateChatGifMessage,
	CreateChatImageMessage,
	CreateCoinsOrder,
	CreateComment,
	CreateCustomEmoji,
	CreateEconOrder,
	CreateEconPayment,
	CreateMediaUploadLease,
	CreatePredictionTournament,
	CreateProfilePostWithVideo,
	CreateRandomAvatar,
	CreateRecurringScheduledPost,
	CreateScheduledPostLink,
	CreateStandaloneScheduledPost,
	CreateStorefrontOrder,
	CreateSubredditPostWithVideo,
	CreatorStats,
	DeleteChatMessage,
	DeleteComment,
	DeleteCustomEmoji,
	DeleteInboxNotifications,
	DeletePost,
	DeleteScheduledPost,
	DeleteSocialLinks,
	DestroyInviteLink,
	DiscoverFeedElements,
	DownloadAvatar,
	DownvotedPosts,
	EconAvatarMarketingEvents,
	EconSpecialEventsWithFreeAwards,
	EndPredictionTournament,
	ExposeExperiments,
	FetchActiveCoinSale,
	FindDirectRoom,
	FollowedByRedditors,
	GenerateCustomEmojiUploadLease,
	GeoContributableSubreddits,
	GetAchievementFlairsStatus,
	GetAllExperimentVariants,
	GetAvatarStorefront,
	GetAvatarStorefrontArtistWithListings,
	GetAvatarStorefrontDynamic,
	GetAvatarStorefrontDynamicLayout,
	GetAvatarStorefrontLegacy,
	GetChatMessageReactionIcons,
	GetCommentById,
	GetCommentByIdWithChildren,
	GetCustomEmojis,
	GetCustomEmojisStatus,
	GetInboxNotificationFeed,
	GetInboxNotificationFeedForReceiveEvents,
	GetInventoryItemsByIds,
	GetLinkTitle,
	GetMatrixChatUsersByIds,
	GetModPnSettingsLayout,
	GetMuxedMP4,
	GetPostRequirements,
	GetPredictionChipPackages,
	GetPredictionTokens,
	GetPredictionTournaments,
	GetStorefrontAuthorsOfListings,
	GetStorefrontAvatarBuilderCatalog,
	GetStorefrontListingItemById,
	GetSubredditAchievementFlairs,
	GetSubredditChannels,
	GetSubredditExperiment,
	GetSubredditGeoPlaceBySubredditId,
	GetSurveyServices,
	GetTopKarmaSubreddits,
	GetTopicsRecommendation,
	GetTrendingChatGifs,
	GetUserAchievementFlairs,
	GetWelcomeMessageForSubreddit,
	GildComment,
	GildPost,
	GiveAward,
	GlobalProductOffers,
	HandleGoogleBilling,
	HiddenPosts,
	HideAwardOnTarget,
	HomeElements,
	IdentityPowerups,
	InterestTopics,
	InterestTopicsByIds,
	IsPredictionCreationAllowed,
	IsUsernameAvailable,
	JoinChatChannelWithInviteLink,
	ModQueueItemsWithSort,
	ModQueueNewItemCount,
	ModRecommendedSubreddits,
	MultiredditByPath,
	MultiredditPosts,
	MyAuthoredMultireddits,
	MyMultireddits,
	MySubscriptionAndCoins,
	NearbySubreddits,
	NewsFeed,
	OnboardingPracticeFeed,
	PersonalizedYearInReview,
	PollVote,
	PopularFeedElements,
	PostComments,
	PostSetPost,
	PostSetSharedTo,
	PostsByIds,
	Profile,
	ProfileTrophies,
	ReOrderSocialLinks,
	ReallocatePowerups,
	RecommendedLinkedPosts,
	RecommendedMediaFeed,
	RecommendedSimilarPosts,
	RecommendedVideoPosts,
	RedditorPowerups,
	RedditorsPowerups,
	RemoveAward,
	ReportAward,
	ReportChatMessage,
	ResolvePrediction,
	SavedComments,
	SavedPosts,
	ScheduledPostsForSubreddit,
	SearchChatGifs,
	SearchUserForBlock,
	SetDefaultEmailPreferences,
	SetSocialLinks,
	SortedUsableAwardsForProfile,
	SortedUsableAwardsWithTags,
	StickyPostsForSubreddit,
	SubmitMediaUpload,
	SubmitScheduledPostNow,
	SubredditChannelsDisableChannels,
	SubredditChannelsEnableChannels,
	SubredditFeedElements,
	SubredditPostRequirements,
	SubredditPowerupTierAndBenefits,
	SubredditTopPredictors,
	SubredditTopSupporters,
	SubredditWikiIndex,
	SubredditWikiPage,
	SubredditsPowerupBenefits,
	SuggestSubredditGeoPlace,
	SuggestedUsernames,
	TopicBySlug,
	UpdateAccountGender,
	UpdateAchievementFlairPreference,
	UpdateChatMessageReaction,
	UpdateComment,
	UpdateCommentDistinguishState,
	UpdateCommentFollowState,
	UpdateCommentSaveState,
	UpdateCommentVoteState,
	UpdateCommunityDiscoveryPreference,
	UpdateCrowdControlFilter,
	UpdateInboxActivitySeenState,
	UpdateModPnSettingStatus,
	UpdateModPnSettingThreshold,
	UpdatePost,
	UpdatePostCrowdControlLevel,
	UpdatePostDistinguishState,
	UpdatePostFollowState,
	UpdatePostHideState,
	UpdatePostNsfwState,
	UpdatePostSaveState,
	UpdatePostSetPostVoteState,
	UpdatePostVoteState,
	UpdatePowerupsSettings,
	UpdatePredictionTournament,
	UpdateRecommendationPreferences,
	UpdateRedditorBlockState,
	UpdateRedditorFriendState,
	UpdateScheduledPost,
	UpdateSocialLinks,
	UpdateSubredditCountrySettings,
	UpdateTopicPreferences,
	UpdateVideoContentPermissionSettings,
	UpvotedPosts,
	UserAvatarInfo,
	UserCoinsInSubredditProducts,
	UserSubmittedPostSets,
	UserSubmittedPosts,
	UsernameAndExperiments,
	VotePrediction,
}